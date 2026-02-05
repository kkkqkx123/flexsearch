use crate::r#type::EncoderOptions;
use crate::error::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

lazy_static::lazy_static! {
    static ref WHITESPACE: Regex = Regex::new(r"[^\p{L}\p{N}]+").unwrap();
    static ref NORMALIZE: Regex = Regex::new(r"[\u{0300}-\u{036f}]").unwrap();
    static ref NUMERIC_SPLIT_LENGTH: Regex = Regex::new(r"(\d{3})").unwrap();
    static ref NUMERIC_SPLIT_PREV_CHAR: Regex = Regex::new(r"(\D)(\d{3})").unwrap();
    static ref NUMERIC_SPLIT_NEXT_CHAR: Regex = Regex::new(r"(\d{3})(\D)").unwrap();
}

#[derive(Clone)]
pub struct Encoder {
    pub normalize: NormalizeOption,
    pub split: SplitOption,
    pub numeric: bool,
    pub prepare: Option<Arc<dyn Fn(String) -> String + Send + Sync>>,
    pub finalize: Option<Arc<dyn Fn(Vec<String>) -> Option<Vec<String>> + Send + Sync>>,
    pub filter: Option<FilterOption>,
    pub dedupe: bool,
    pub matcher: Option<HashMap<String, String>>,
    pub mapper: Option<HashMap<char, char>>,
    pub stemmer: Option<HashMap<String, String>>,
    pub replacer: Option<Vec<(Regex, String)>>,
    pub minlength: usize,
    pub maxlength: usize,
    pub rtl: bool,
    pub cache: Option<Cache>,
}

#[derive(Clone)]
pub enum NormalizeOption {
    Bool(bool),
    Function(Arc<dyn Fn(String) -> String + Send + Sync>),
}

#[derive(Clone)]
pub enum SplitOption {
    String(String),
    Regex(Regex),
    Bool(bool),
}

#[derive(Clone)]
pub enum FilterOption {
    Set(HashSet<String>),
    Function(Arc<dyn Fn(&str) -> bool + Send + Sync>),
}

#[derive(Clone)]
pub struct Cache {
    pub size: usize,
    pub cache_enc: Arc<RwLock<lru::LruCache<String, Vec<String>>>>,
    pub cache_term: Arc<RwLock<lru::LruCache<String, String>>>,
    pub cache_enc_length: usize,
    pub cache_term_length: usize,
}

impl Encoder {
    pub fn new(options: EncoderOptions) -> Self {
        let normalize = match options.normalize {
            Some(true) => NormalizeOption::Bool(true),
            Some(false) => NormalizeOption::Bool(false),
            None => NormalizeOption::Bool(true),
        };

        let split = if let Some(split) = options.split {
            if split.is_empty() {
                SplitOption::String(String::new())
            } else {
                SplitOption::String(split)
            }
        } else {
            SplitOption::Regex(WHITESPACE.clone())
        };

        let numeric = options.numeric.unwrap_or(true);

        let filter = options.filter.map(|filter| {
            if filter.is_empty() {
                FilterOption::Set(HashSet::new())
            } else {
                FilterOption::Set(filter.into_iter().collect())
            }
        });

        let dedupe = options.dedupe.unwrap_or(true);

        let matcher = options.matcher;
        let mapper = options.mapper;
        let stemmer = options.stemmer;

        let replacer = options.replacer.map(|replacer| {
            replacer
                .into_iter()
                .map(|(pattern, replacement)| {
                    let regex = Regex::new(&pattern).unwrap();
                    (regex, replacement)
                })
                .collect()
        });

        let minlength = options.minlength.unwrap_or(1);
        let maxlength = options.maxlength.unwrap_or(1024);
        let rtl = options.rtl.unwrap_or(false);

        let cache = if options.cache.unwrap_or(true) {
            Some(Cache {
                size: 200_000,
                cache_enc: Arc::new(RwLock::new(lru::LruCache::unbounded())),
                cache_term: Arc::new(RwLock::new(lru::LruCache::unbounded())),
                cache_enc_length: 128,
                cache_term_length: 128,
            })
        } else {
            None
        };

        Encoder {
            normalize,
            split,
            numeric,
            prepare: None,
            finalize: None,
            filter,
            dedupe,
            matcher,
            mapper,
            stemmer,
            replacer,
            minlength,
            maxlength,
            rtl,
            cache,
        }
    }

    pub fn encode(&self, str: &str) -> Result<Vec<String>> {
        let mut s = str.to_string();

        if let Some(cache) = &self.cache {
            if s.len() <= cache.cache_enc_length {
                if let Ok(cache_enc) = cache.cache_enc.read() {
                    if let Some(result) = cache_enc.peek(&s) {
                        return Ok(result.clone());
                    }
                }
            }
        }

        s = self.apply_normalize(&s);

        if let Some(prepare) = &self.prepare {
            s = prepare(s);
        }

        if self.numeric && s.len() > 3 {
            s = NUMERIC_SPLIT_PREV_CHAR
                .replace_all(&s, "$1 $2")
                .to_string();
            s = NUMERIC_SPLIT_NEXT_CHAR
                .replace_all(&s, "$1 $2")
                .to_string();
            s = NUMERIC_SPLIT_LENGTH.replace_all(&s, "$1 ").to_string();
        }

        let words = self.apply_split(&s);

        let skip = !self.has_transformations();

        let mut final_terms = Vec::new();
        let mut dupes = HashSet::new();
        let mut last_term = String::new();
        let mut last_term_enc = String::new();

        for word in words {
            let base = word.clone();

            if word.is_empty() {
                continue;
            }

            if word.len() < self.minlength || word.len() > self.maxlength {
                continue;
            }

            if self.dedupe {
                if dupes.contains(&word) {
                    continue;
                }
                dupes.insert(word.clone());
            } else {
                if last_term == word {
                    continue;
                }
                last_term = word.clone();
            }

            if skip {
                final_terms.push(word);
                continue;
            }

            if let Some(filter) = &self.filter {
                if !self.apply_filter(filter, &word) {
                    continue;
                }
            }

            let mut word = word;

            if let Some(cache) = &self.cache {
                if base.len() <= cache.cache_term_length {
                    if let Ok(cache_term) = cache.cache_term.read() {
                        if let Some(tmp) = cache_term.peek(&base) {
                            if !tmp.is_empty() {
                                final_terms.push(tmp.clone());
                            }
                            continue;
                        }
                    }
                }
            }

            if let Some(stemmer) = &self.stemmer {
                word = self.apply_stemmer(&word, stemmer);
            }

            if self.mapper.is_some() || (self.dedupe && word.len() > 1) {
                word = self.apply_mapper(&word);
            }

            if let Some(matcher) = &self.matcher {
                word = self.apply_matcher(&word, matcher);
            }

            if let Some(replacer) = &self.replacer {
                word = self.apply_replacer(&word, replacer);
            }

            if let Some(cache) = &self.cache {
                if base.len() <= cache.cache_term_length {
                    if let Ok(mut cache_term) = cache.cache_term.write() {
                        cache_term.put(base.clone(), word.clone());
                    }
                }
            }

            if !word.is_empty() {
                if word != base {
                    if self.dedupe {
                        if dupes.contains(&word) {
                            continue;
                        }
                        dupes.insert(word.clone());
                    } else {
                        if last_term_enc == word {
                            continue;
                        }
                        last_term_enc = word.clone();
                    }
                }
                final_terms.push(word);
            }
        }

        if let Some(finalize) = &self.finalize {
            if let Some(result) = finalize(final_terms.clone()) {
                final_terms = result;
            }
        }

        if let Some(cache) = &self.cache {
            if s.len() <= cache.cache_enc_length {
                if let Ok(mut cache_enc) = cache.cache_enc.write() {
                    cache_enc.put(s.clone(), final_terms.clone());
                }
            }
        }

        Ok(final_terms)
    }

    fn apply_normalize(&self, str: &str) -> String {
        match &self.normalize {
            NormalizeOption::Bool(true) => {
                use unicode_normalization::UnicodeNormalization;
                NORMALIZE.replace_all(str.chars().nfkd().collect::<String>().as_str(), "")
                    .to_lowercase()
            }
            NormalizeOption::Bool(false) => str.to_lowercase(),
            NormalizeOption::Function(func) => func(str.to_string()),
        }
    }

    fn apply_split(&self, str: &str) -> Vec<String> {
        match &self.split {
            SplitOption::String(s) if s.is_empty() => vec![str.to_string()],
            SplitOption::String(s) => str.split(s).map(|s| s.to_string()).collect(),
            SplitOption::Regex(regex) => {
                regex.split(str).map(|s| s.to_string()).collect()
            }
            SplitOption::Bool(false) => vec![str.to_string()],
            SplitOption::Bool(true) => {
                WHITESPACE.split(str).map(|s| s.to_string()).collect()
            }
        }
    }

    fn has_transformations(&self) -> bool {
        self.filter.is_some()
            || self.mapper.is_some()
            || self.matcher.is_some()
            || self.stemmer.is_some()
            || self.replacer.is_some()
    }

    fn apply_filter(&self, filter: &FilterOption, word: &str) -> bool {
        match filter {
            FilterOption::Set(set) => !set.contains(word),
            FilterOption::Function(func) => func(word),
        }
    }

    fn apply_stemmer(&self, word: &str, stemmer: &HashMap<String, String>) -> String {
        let mut word = word.to_string();
        let mut old = String::new();

        while old != word && word.len() > 2 {
            old = word.clone();
            for (key, value) in stemmer {
                if word.len() > key.len() && word.ends_with(key) {
                    word = format!("{}{}", &word[..word.len() - key.len()], value);
                    break;
                }
            }
        }

        word
    }

    fn apply_mapper(&self, word: &str) -> String {
        let mut result = String::new();
        let mut prev = String::new();

        for char in word.chars() {
            if char.to_string() != prev || !self.dedupe {
                let tmp = self.mapper.as_ref().and_then(|m| m.get(&char).copied());
                if let Some(mapped) = tmp {
                    if mapped.to_string() != prev || !self.dedupe {
                        result.push(mapped);
                        prev = mapped.to_string();
                    }
                } else {
                    result.push(char);
                    prev = char.to_string();
                }
            }
        }

        result
    }

    fn apply_matcher(&self, word: &str, matcher: &HashMap<String, String>) -> String {
        let mut result = word.to_string();

        for (key, value) in matcher {
            result = result.replace(key, value);
        }

        result
    }

    fn apply_replacer(&self, word: &str, replacer: &[(Regex, String)]) -> String {
        let mut result = word.to_string();

        for (regex, replacement) in replacer {
            result = regex.replace_all(&result, replacement).to_string();
        }

        result
    }

    pub fn add_stemmer(&mut self, match_str: String, replace: String) {
        if self.stemmer.is_none() {
            self.stemmer = Some(HashMap::new());
        }
        if let Some(stemmer) = &mut self.stemmer {
            stemmer.insert(match_str, replace);
        }
        if let Some(cache) = &mut self.cache {
            if let Ok(mut cache_enc) = cache.cache_enc.write() {
                cache_enc.clear();
            }
            if let Ok(mut cache_term) = cache.cache_term.write() {
                cache_term.clear();
            }
        }
    }

    pub fn add_filter(&mut self, term: String) {
        if self.filter.is_none() {
            self.filter = Some(FilterOption::Set(HashSet::new()));
        }
        if let FilterOption::Set(set) = self.filter.as_mut().unwrap() {
            set.insert(term);
        }
        if let Some(cache) = &mut self.cache {
            if let Ok(mut cache_enc) = cache.cache_enc.write() {
                cache_enc.clear();
            }
            if let Ok(mut cache_term) = cache.cache_term.write() {
                cache_term.clear();
            }
        }
    }

    pub fn add_mapper(&mut self, char_match: char, char_replace: char) {
        if self.mapper.is_none() {
            self.mapper = Some(HashMap::new());
        }
        if let Some(mapper) = &mut self.mapper {
            mapper.insert(char_match, char_replace);
        }
        if let Some(cache) = &mut self.cache {
            if let Ok(mut cache_enc) = cache.cache_enc.write() {
                cache_enc.clear();
            }
            if let Ok(mut cache_term) = cache.cache_term.write() {
                cache_term.clear();
            }
        }
    }

    pub fn add_matcher(&mut self, match_str: String, replace: String) {
        if self.matcher.is_none() {
            self.matcher = Some(HashMap::new());
        }
        if let Some(matcher) = &mut self.matcher {
            matcher.insert(match_str, replace);
        }
        if let Some(cache) = &mut self.cache {
            if let Ok(mut cache_enc) = cache.cache_enc.write() {
                cache_enc.clear();
            }
            if let Ok(mut cache_term) = cache.cache_term.write() {
                cache_term.clear();
            }
        }
    }

    pub fn add_replacer(&mut self, regex: Regex, replace: String) {
        if self.replacer.is_none() {
            self.replacer = Some(Vec::new());
        }
        if let Some(replacer) = &mut self.replacer {
            replacer.push((regex, replace));
        }
        if let Some(cache) = &mut self.cache {
            if let Ok(mut cache_enc) = cache.cache_enc.write() {
                cache_enc.clear();
            }
            if let Ok(mut cache_term) = cache.cache_term.write() {
                cache_term.clear();
            }
        }
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Encoder::new(EncoderOptions::default())
    }
}

pub fn fallback_encoder(str: &str) -> Vec<String> {
    use unicode_normalization::UnicodeNormalization;
    NORMALIZE.replace_all(str.chars().nfkd().collect::<String>().as_str(), "")
        .to_lowercase()
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_default() {
        let encoder = Encoder::default();
        let result = encoder.encode("Hello World").unwrap();
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_encoder_normalize() {
        let encoder = Encoder::default();
        let result = encoder.encode("Héllo Wörld").unwrap();
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_encoder_numeric() {
        let encoder = Encoder::default();
        let result = encoder.encode("123456").unwrap();
        assert_eq!(result, vec!["123", "456"]);
    }

    #[test]
    fn test_encoder_minlength() {
        let options = EncoderOptions {
            minlength: Some(3),
            ..Default::default()
        };
        let encoder = Encoder::new(options);
        let result = encoder.encode("hi hello world").unwrap();
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_encoder_dedupe() {
        let options = EncoderOptions {
            dedupe: Some(true),
            ..Default::default()
        };
        let encoder = Encoder::new(options);
        let result = encoder.encode("hello hello world world").unwrap();
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_encoder_filter() {
        let options = EncoderOptions {
            filter: Some(vec!["the".to_string(), "and".to_string()]),
            ..Default::default()
        };
        let encoder = Encoder::new(options);
        let result = encoder.encode("the cat and the dog").unwrap();
        assert_eq!(result, vec!["cat", "dog"]);
    }

    #[test]
    fn test_encoder_stemmer() {
        let mut options = EncoderOptions::default();
        let mut stemmer = HashMap::new();
        stemmer.insert("ing".to_string(), "".to_string());
        options.stemmer = Some(stemmer);
        let encoder = Encoder::new(options);
        let result = encoder.encode("running jumping").unwrap();
        assert_eq!(result, vec!["run", "jump"]);
    }

    #[test]
    fn test_encoder_mapper() {
        let mut options = EncoderOptions::default();
        options.dedupe = Some(false); // Disable dedupe for this test
        let mut mapper = HashMap::new();
        mapper.insert('a', 'b');
        options.mapper = Some(mapper);
        let encoder = Encoder::new(options);
        let result = encoder.encode("apple").unwrap();
        assert_eq!(result, vec!["bpple"]);
    }

    #[test]
    fn test_encoder_matcher() {
        let mut options = EncoderOptions::default();
        let mut matcher = HashMap::new();
        matcher.insert("color".to_string(), "colour".to_string());
        options.matcher = Some(matcher);
        let encoder = Encoder::new(options);
        let result = encoder.encode("color").unwrap();
        assert_eq!(result, vec!["colour"]);
    }

    #[test]
    fn test_fallback_encoder() {
        let result = fallback_encoder("Hello World");
        assert_eq!(result, vec!["hello", "world"]);
    }
}

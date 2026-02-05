use std::collections::HashMap;
use regex::Regex;
use crate::r#type::EncoderOptions;
use crate::charset::latin::advanced;

pub fn get_compact_replacer() -> Vec<(Regex, String)> {
    vec![
        (Regex::new(r"(?!^)[aeo]").unwrap(), "".to_string()),
    ]
}

pub fn get_compact_replacer_strings() -> Vec<(String, String)> {
    vec![
        (r"(?!^)[aeo]", ""),
    ].into_iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
}

pub fn get_charset_latin_extra() -> EncoderOptions {
    let mut replacer = advanced::get_replacer_strings();
    replacer.extend(get_compact_replacer_strings());
    
    let mut mapper = HashMap::new();
    // Add soundex mappings for single characters
    mapper.insert('b', 'p');
    mapper.insert('v', 'f');
    mapper.insert('w', 'f');
    mapper.insert('z', 's');
    mapper.insert('x', 's');
    mapper.insert('d', 't');
    mapper.insert('n', 'm');
    mapper.insert('c', 'k');
    mapper.insert('g', 'k');
    mapper.insert('j', 'k');
    mapper.insert('q', 'k');
    mapper.insert('i', 'e');
    mapper.insert('y', 'e');
    mapper.insert('u', 'o');
    
    EncoderOptions {
        mapper: Some(mapper),
        replacer: Some(replacer),
        matcher: Some(advanced::get_matcher()),
        rtl: Some(false),
        dedupe: Some(true),
        split: None,
        numeric: Some(true),
        normalize: Some(true),
        prepare: None,
        finalize: None,
        filter: None,
        stemmer: None,
        minlength: Some(1),
        maxlength: Some(1024),
        cache: Some(true),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charset_latin_extra() {
        let options = get_charset_latin_extra();
        assert!(options.mapper.is_some());
        assert!(options.replacer.is_some());
        assert!(options.matcher.is_some());
        
        let replacer = options.replacer.unwrap();
        // Should have more replacers than just advanced due to compact
        assert!(replacer.len() > 2);
    }
}
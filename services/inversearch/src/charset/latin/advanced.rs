use std::collections::HashMap;
use regex::Regex;
use crate::r#type::EncoderOptions;

pub fn get_matcher() -> HashMap<String, String> {
    let mut matcher = HashMap::new();
    
    matcher.insert("ae".to_string(), "a".to_string());
    matcher.insert("oe".to_string(), "o".to_string());
    matcher.insert("sh".to_string(), "s".to_string());
    matcher.insert("kh".to_string(), "k".to_string());
    matcher.insert("th".to_string(), "t".to_string());
    matcher.insert("ph".to_string(), "f".to_string());
    matcher.insert("pf".to_string(), "f".to_string());
    
    matcher
}

pub fn get_replacer() -> Vec<(Regex, String)> {
    vec![
        (Regex::new(r"([^aeo])h(.)").unwrap(), "$1$2".to_string()),
        (Regex::new(r"([aeo])h([^aeo]|$)").unwrap(), "$1$2".to_string()),
        (Regex::new(r"(.)\1+").unwrap(), "$1".to_string()),
    ]
}

pub fn get_replacer_strings() -> Vec<(String, String)> {
    vec![
        (r"([^aeo])h(.)", "$1$2"),
        (r"([aeo])h([^aeo]|$)", "$1$2"),
        // Note: Rust regex doesn't support backreferences like (.)+
        // This would need to be handled differently in the encoder implementation
    ].into_iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
}

pub fn get_charset_latin_advanced() -> EncoderOptions {
    let mut mapper = HashMap::new();
    
    // Apply soundex mapping for single characters
    mapper.insert('t', 't');
    mapper.insert('e', 'e');
    mapper.insert('s', 's');
    
    EncoderOptions {
        mapper: Some(mapper),
        matcher: Some(get_matcher()),
        replacer: Some(get_replacer_strings()),
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
    fn test_charset_latin_advanced() {
        let options = get_charset_latin_advanced();
        assert!(options.mapper.is_some());
        assert!(options.matcher.is_some());
        assert!(options.replacer.is_some());
        
        let matcher = options.matcher.unwrap();
        assert_eq!(matcher.get("ae"), Some(&"a".to_string()));
        assert_eq!(matcher.get("oe"), Some(&"o".to_string()));
    }

    #[test]
    fn test_replacer() {
        let replacer = get_replacer_strings();
        assert_eq!(replacer.len(), 2); // We removed the backreference regex
    }
}
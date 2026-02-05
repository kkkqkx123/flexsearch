use std::collections::HashMap;
use crate::r#type::EncoderOptions;

pub fn get_soundex_map() -> HashMap<char, char> {
    let mut map = HashMap::new();
    
    map.insert('b', 'p');
    map.insert('v', 'f');
    map.insert('w', 'f');
    map.insert('z', 's');
    map.insert('x', 's');
    map.insert('d', 't');
    map.insert('n', 'm');
    map.insert('c', 'k');
    map.insert('g', 'k');
    map.insert('j', 'k');
    map.insert('q', 'k');
    map.insert('i', 'e');
    map.insert('y', 'e');
    map.insert('u', 'o');
    
    map
}

pub fn get_charset_latin_balance() -> EncoderOptions {
    EncoderOptions {
        mapper: Some(get_soundex_map()),
        rtl: Some(false),
        dedupe: Some(true),
        split: None,
        numeric: Some(true),
        normalize: Some(true),
        prepare: None,
        finalize: None,
        filter: None,
        matcher: None,
        stemmer: None,
        replacer: None,
        minlength: Some(1),
        maxlength: Some(1024),
        cache: Some(true),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charset_latin_balance() {
        let options = get_charset_latin_balance();
        assert!(options.mapper.is_some());
        let mapper = options.mapper.unwrap();
        assert_eq!(mapper.get(&'b'), Some(&'p'));
        assert_eq!(mapper.get(&'v'), Some(&'f'));
    }
}
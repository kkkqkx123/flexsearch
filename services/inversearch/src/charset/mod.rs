use std::collections::HashMap;

pub fn get_charset_polyfill() -> HashMap<char, &'static str> {
    let mut map = HashMap::new();

    map.insert('ª', "a");
    map.insert('²', "2");
    map.insert('³', "3");
    map.insert('¹', "1");
    map.insert('º', "o");
    map.insert('¼', "1⁄4");
    map.insert('½', "1⁄2");
    map.insert('¾', "3⁄4");
    map.insert('à', "a");
    map.insert('á', "a");
    map.insert('â', "a");
    map.insert('ã', "a");
    map.insert('ä', "a");
    map.insert('å', "a");
    map.insert('ç', "c");
    map.insert('è', "e");
    map.insert('é', "e");
    map.insert('ê', "e");
    map.insert('ë', "e");
    map.insert('ì', "i");
    map.insert('í', "i");
    map.insert('î', "i");
    map.insert('ï', "i");
    map.insert('ñ', "n");
    map.insert('ò', "o");
    map.insert('ó', "o");
    map.insert('ô', "o");
    map.insert('õ', "o");
    map.insert('ö', "o");
    map.insert('ù', "u");
    map.insert('ú', "u");
    map.insert('û', "u");
    map.insert('ü', "u");
    map.insert('ý', "y");
    map.insert('ÿ', "y");
    map.insert('ā', "a");
    map.insert('ă', "a");
    map.insert('ą', "a");
    map.insert('ć', "c");
    map.insert('ĉ', "c");
    map.insert('ċ', "c");
    map.insert('č', "c");
    map.insert('ď', "d");
    map.insert('ē', "e");
    map.insert('ĕ', "e");
    map.insert('ė', "e");
    map.insert('ę', "e");
    map.insert('ě', "e");
    map.insert('ĝ', "g");
    map.insert('ğ', "g");
    map.insert('ġ', "g");
    map.insert('ģ', "g");
    map.insert('ĥ', "h");
    map.insert('ĩ', "i");
    map.insert('ī', "i");
    map.insert('ĭ', "i");
    map.insert('į', "i");
    map.insert('ĳ', "ij");
    map.insert('ĵ', "j");
    map.insert('ķ', "k");
    map.insert('ĺ', "l");
    map.insert('ļ', "l");
    map.insert('ľ', "l");
    map.insert('ŀ', "l");
    map.insert('ń', "n");
    map.insert('ņ', "n");
    map.insert('ň', "n");
    map.insert('ŉ', "n");
    map.insert('ō', "o");
    map.insert('ŏ', "o");
    map.insert('ő', "o");
    map.insert('ŕ', "r");
    map.insert('ŗ', "r");
    map.insert('ř', "r");
    map.insert('ś', "s");
    map.insert('ŝ', "s");
    map.insert('ş', "s");
    map.insert('š', "s");
    map.insert('ţ', "t");
    map.insert('ť', "t");
    map.insert('ũ', "u");
    map.insert('ū', "u");
    map.insert('ŭ', "u");
    map.insert('ů', "u");
    map.insert('ű', "u");
    map.insert('ų', "u");
    map.insert('ŵ', "w");
    map.insert('ŷ', "y");
    map.insert('ź', "z");
    map.insert('ż', "z");
    map.insert('ž', "z");
    map.insert('ſ', "s");
    map.insert('ơ', "o");
    map.insert('ư', "u");
    map.insert('ǆ', "dz");
    map.insert('ǉ', "lj");
    map.insert('ǌ', "nj");
    map.insert('ǎ', "a");
    map.insert('ǐ', "i");
    map.insert('ǒ', "o");
    map.insert('ǔ', "u");
    map.insert('ǖ', "u");
    map.insert('ǘ', "u");
    map.insert('ǚ', "u");
    map.insert('ǜ', "u");
    map.insert('ǟ', "a");
    map.insert('ǡ', "a");
    map.insert('ǣ', "ae");
    map.insert('æ', "ae");
    map.insert('ǽ', "ae");
    map.insert('ǧ', "g");
    map.insert('ǩ', "k");
    map.insert('ǫ', "o");
    map.insert('ǭ', "o");
    map.insert('ǯ', "ʒ");
    map.insert('ǰ', "j");
    map.insert('ǳ', "dz");
    map.insert('ǵ', "g");
    map.insert('ǹ', "n");
    map.insert('ǻ', "a");
    map.insert('ǿ', "ø");
    map.insert('ȁ', "a");
    map.insert('ȃ', "a");
    map.insert('ȅ', "e");
    map.insert('ȇ', "e");
    map.insert('ȉ', "i");
    map.insert('ȋ', "i");
    map.insert('ȍ', "o");
    map.insert('ȏ', "o");
    map.insert('ȑ', "r");
    map.insert('ȓ', "r");
    map.insert('ȕ', "u");
    map.insert('ȗ', "u");
    map.insert('ș', "s");
    map.insert('ț', "t");
    map.insert('ȟ', "h");
    map.insert('ȧ', "a");
    map.insert('ȩ', "e");
    map.insert('ȫ', "o");
    map.insert('ȭ', "o");
    map.insert('ȯ', "o");
    map.insert('ȱ', "o");
    map.insert('ȳ', "y");
    map.insert('ʰ', "h");
    map.insert('ʱ', "h");
    map.insert('ɦ', "h");
    map.insert('ʲ', "j");
    map.insert('ʳ', "r");
    map.insert('ʴ', "ɹ");
    map.insert('ʵ', "ɻ");
    map.insert('ʶ', "ʁ");
    map.insert('ʷ', "w");
    map.insert('ʸ', "y");
    map.insert('ˠ', "ɣ");
    map.insert('ˡ', "l");
    map.insert('ˢ', "s");
    map.insert('ˣ', "x");
    map.insert('ˤ', "ʕ");
    map.insert('ΐ', "ι");
    map.insert('ά', "α");
    map.insert('έ', "ε");
    map.insert('ή', "η");
    map.insert('ί', "ι");
    map.insert('ΰ', "υ");
    map.insert('ϊ', "ι");
    map.insert('ϋ', "υ");
    map.insert('ό', "ο");
    map.insert('ύ', "υ");
    map.insert('ώ', "ω");
    map.insert('ϐ', "β");
    map.insert('ϑ', "θ");
    map.insert('ϒ', "Υ");
    map.insert('ϓ', "Υ");
    map.insert('ϔ', "Υ");
    map.insert('ϕ', "φ");
    map.insert('ϖ', "π");
    map.insert('ϰ', "κ");
    map.insert('ϱ', "ρ");
    map.insert('ϲ', "ς");
    map.insert('ϵ', "ε");
    map.insert('й', "и");
    map.insert('ѐ', "е");
    map.insert('ё', "е");
    map.insert('ѓ', "г");
    map.insert('ї', "і");
    map.insert('ќ', "к");
    map.insert('ѝ', "и");
    map.insert('ў', "у");
    map.insert('ѷ', "ѵ");
    map.insert('ӂ', "ж");
    map.insert('ӑ', "а");
    map.insert('ӓ', "а");
    map.insert('ӗ', "е");
    map.insert('ӛ', "ә");
    map.insert('ӝ', "ж");
    map.insert('ӟ', "з");
    map.insert('ӣ', "и");
    map.insert('ӥ', "и");
    map.insert('ӧ', "о");
    map.insert('ӫ', "ө");
    map.insert('ӭ', "э");
    map.insert('ӯ', "у");
    map.insert('ӱ', "у");
    map.insert('ӳ', "у");
    map.insert('ӵ', "ч");

    map
}

pub fn normalize_charset(str: &str) -> String {
    let polyfill = get_charset_polyfill();
    let mut result = String::new();

    for char in str.chars() {
        if let Some(replacement) = polyfill.get(&char) {
            result.push_str(replacement);
        } else {
            result.push(char);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charset_polyfill() {
        let polyfill = get_charset_polyfill();
        assert_eq!(polyfill.get(&'à'), Some(&"a"));
        assert_eq!(polyfill.get(&'é'), Some(&"e"));
        assert_eq!(polyfill.get(&'ñ'), Some(&"n"));
    }

    #[test]
    fn test_normalize_charset() {
        let result = normalize_charset("Héllo Wörld");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_normalize_charset_multiple() {
        let result = normalize_charset("café naïve");
        assert_eq!(result, "cafe naive");
    }

    #[test]
    fn test_normalize_charset_cyrillic() {
        let result = normalize_charset("йё");
        assert_eq!(result, "ие");
    }

    #[test]
    fn test_normalize_charset_greek() {
        let result = normalize_charset("άέή");
        assert_eq!(result, "αεη");
    }
}

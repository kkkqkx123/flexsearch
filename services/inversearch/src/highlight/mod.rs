use crate::r#type::{HighlightOptions, HighlightBoundaryOptions, HighlightEllipsisOptions};
use crate::encoder::Encoder;
use crate::error::Result;
use std::collections::HashMap;

pub struct Highlighter {
    pub template: String,
    pub markup_open: String,
    pub markup_close: String,
    pub boundary: Option<HighlightBoundaryOptions>,
    pub clip: bool,
    pub merge: Option<String>,
    pub ellipsis: Option<HighlightEllipsisOptions>,
}

impl Highlighter {
    pub fn new(options: &HighlightOptions) -> Result<Self> {
        let template = options.template.clone();
        
        let markup_open_pos = template.find("$1")
            .ok_or_else(|| crate::error::InversearchError::Encoder(
                crate::error::EncoderError::Encoding(
                    "Invalid highlight template. The replacement pattern \"$1\" was not found".to_string()
                )
            ))?;

        let markup_open = template[..markup_open_pos].to_string();
        let markup_close = template[markup_open_pos + 2..].to_string();

        let clip = options.clip.unwrap_or(true);
        let merge = if clip && !markup_open.is_empty() && !markup_close.is_empty() {
            Some(format!("{} {}", markup_close, markup_open))
        } else {
            None
        };

        let ellipsis = options.ellipsis.as_ref().map(|e| {
            let ellipsis_template = e.template.clone();
            let ellipsis_markup_length = ellipsis_template.len() - 2;
            let ellipsis_pattern = e.pattern.clone();
            HighlightEllipsisOptions {
                template: ellipsis_template,
                pattern: ellipsis_pattern,
            }
        });

        Ok(Highlighter {
            template,
            markup_open,
            markup_close,
            boundary: options.boundary.clone(),
            clip,
            merge,
            ellipsis,
        })
    }

    pub fn highlight_fields(
        &self,
        query: &str,
        content: &str,
        encoder: &Encoder,
    ) -> Result<String> {
        let query_terms = encoder.encode(query)?;
        let doc_terms: Vec<&str> = content.split_whitespace().collect();

        let mut highlighted = Vec::new();
        let mut pos_matches = Vec::new();
        let mut pos_first_match = -1i32;
        let mut pos_last_match = -1i32;
        let mut length_matches_all = 0usize;

        for (k, doc_term) in doc_terms.iter().enumerate() {
            let doc_term_trimmed = doc_term.trim();
            if doc_term_trimmed.is_empty() {
                continue;
            }

            let doc_enc = encoder.encode(doc_term_trimmed)?;
            let doc_enc_str = if doc_enc.len() > 1 {
                doc_enc.join(" ")
            } else if !doc_enc.is_empty() {
                doc_enc[0].clone()
            } else {
                String::new()
            };

            let mut found = false;

            if !doc_enc_str.is_empty() && !doc_term_trimmed.is_empty() {
                let doc_term_len = doc_term_trimmed.len();
                let mut match_str = String::new();
                let mut match_length = 0usize;

                for query_term in &query_terms {
                    if query_term.is_empty() {
                        continue;
                    }

                    let query_term_len = query_term.len();
                    if match_length > 0 && query_term_len <= match_length {
                        continue;
                    }

                    if let Some(pos) = doc_enc_str.find(query_term) {
                        let prefix = &doc_term_trimmed[..pos];
                        let match_content = &doc_term_trimmed[pos..pos + query_term_len];
                        let _suffix = &doc_term_trimmed[pos + query_term_len..];

                        match_str = format!(
                            "{}{}{}{}",
                            prefix,
                            self.markup_open,
                            match_content,
                            self.markup_close
                        );
                        match_length = query_term_len;
                        found = true;
                    }
                }

                if found {
                    if let Some(_boundary) = &self.boundary {
                        let _boundary_total = _boundary.total.unwrap_or(900000);

                        if pos_first_match < 0 {
                            pos_first_match = highlighted.join(" ").len() as i32;
                        }
                        pos_last_match = (highlighted.join(" ").len() + match_str.len()) as i32;
                        length_matches_all += doc_term_len;
                        pos_matches.push(k);
                    }

                    highlighted.push(match_str);
                }
            }

            if !found {
                highlighted.push(doc_term_trimmed.to_string());
            } else if let Some(_boundary) = &self.boundary {
                let _boundary_total = _boundary.total.unwrap_or(900000);
                if length_matches_all >= _boundary_total {
                    break;
                }
            }
        }

        let result = highlighted.join(" ");

        if let Some(_boundary) = &self.boundary {
            self.apply_boundary(&result, &pos_matches, pos_first_match, pos_last_match, length_matches_all)
        } else {
            Ok(result)
        }
    }

    fn apply_boundary(
        &self,
        str: &str,
        pos_matches: &[usize],
        pos_first_match: i32,
        pos_last_match: i32,
        length_matches_all: usize,
    ) -> Result<String> {
        let boundary = match self.boundary.as_ref() {
            Some(b) => b,
            None => return Err(crate::error::InversearchError::Highlight("No boundary configuration provided".to_string())),
        };
        let boundary_total = boundary.total.unwrap_or(900000);
        let boundary_before = boundary.before.unwrap_or(0);
        let boundary_after = boundary.after.unwrap_or(0);

        let markup_length = pos_matches.len() * (self.template.len() - 2);
        let ellipsis = self.get_ellipsis();
        let ellipsis_length = ellipsis.len();

        let boundary_length = (boundary_total + markup_length - ellipsis_length * 2) as i32;
        let length = pos_last_match - pos_first_match;

        if boundary_before > 0 || boundary_after > 0 || (str.len() - markup_length) > boundary_total {
            let start = if boundary_before > 0 {
                pos_first_match - boundary_before as i32
            } else {
                pos_first_match - ((boundary_length - length) / 2)
            };

            let end = if boundary_after > 0 {
                pos_last_match + boundary_after as i32
            } else {
                start + boundary_length
            };

            let start_usize = std::cmp::max(0, start) as usize;
            let end_usize = std::cmp::min(str.len(), end as usize);

            let result = if start_usize > 0 {
                format!("{}{}{}", ellipsis, &str[start_usize..end_usize], if end_usize < str.len() { ellipsis.clone() } else { String::new() })
            } else {
                format!("{}{}", &str[start_usize..end_usize], if end_usize < str.len() { ellipsis.clone() } else { String::new() })
            };

            Ok(result)
        } else {
            Ok(str.to_string())
        }
    }

    fn get_ellipsis(&self) -> String {
        if let Some(ellipsis) = &self.ellipsis {
            let _ellipsis_markup_length = ellipsis.template.len() - 2;
            let ellipsis_pattern = ellipsis.pattern.as_deref().unwrap_or("...");
            ellipsis.template.replace("$1", ellipsis_pattern)
        } else {
            "...".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r#type::HighlightOptions;

    #[test]
    fn test_highlighter_new() {
        let options = HighlightOptions {
            template: "<b>$1</b>".to_string(),
            boundary: None,
            clip: None,
            merge: None,
            ellipsis: None,
        };
        let highlighter = Highlighter::new(&options).unwrap();
        assert_eq!(highlighter.markup_open, "<b>");
        assert_eq!(highlighter.markup_close, "</b>");
    }

    #[test]
    fn test_highlighter_new_invalid_template() {
        let options = HighlightOptions {
            template: "<b></b>".to_string(),
            boundary: None,
            clip: None,
            merge: None,
            ellipsis: None,
        };
        let result = Highlighter::new(&options);
        assert!(result.is_err());
    }

    #[test]
    fn test_highlight_fields() {
        let options = HighlightOptions {
            template: "<b>$1</b>".to_string(),
            boundary: None,
            clip: None,
            merge: None,
            ellipsis: None,
        };
        let highlighter = Highlighter::new(&options).unwrap();
        let encoder = Encoder::default();
        let result = highlighter.highlight_fields("hello", "hello world", &encoder).unwrap();
        assert_eq!(result, "<b>hello</b> world");
    }

    #[test]
    fn test_highlight_fields_multiple_matches() {
        let options = HighlightOptions {
            template: "<b>$1</b>".to_string(),
            boundary: None,
            clip: None,
            merge: None,
            ellipsis: None,
        };
        let highlighter = Highlighter::new(&options).unwrap();
        let encoder = Encoder::default();
        let result = highlighter.highlight_fields("hello world", "hello world test", &encoder).unwrap();
        assert_eq!(result, "<b>hello</b> <b>world</b> test");
    }

    #[test]
    fn test_highlight_fields_no_match() {
        let options = HighlightOptions {
            template: "<b>$1</b>".to_string(),
            boundary: None,
            clip: None,
            merge: None,
            ellipsis: None,
        };
        let highlighter = Highlighter::new(&options).unwrap();
        let encoder = Encoder::default();
        let result = highlighter.highlight_fields("foo", "hello world", &encoder).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_highlight_fields_with_boundary() {
        let boundary = HighlightBoundaryOptions {
            before: Some(5),
            after: Some(5),
            total: Some(50),
        };
        let options = HighlightOptions {
            template: "<b>$1</b>".to_string(),
            boundary: Some(boundary),
            clip: None,
            merge: None,
            ellipsis: None,
        };
        let highlighter = Highlighter::new(&options).unwrap();
        let encoder = Encoder::default();
        let result = highlighter.highlight_fields("hello", "this is a long text with hello in the middle and more text after", &encoder).unwrap();
        assert!(result.contains("<b>hello</b>"));
    }

    #[test]
    fn test_get_ellipsis() {
        let options = HighlightOptions {
            template: "<b>$1</b>".to_string(),
            boundary: None,
            clip: None,
            merge: None,
            ellipsis: None,
        };
        let highlighter = Highlighter::new(&options).unwrap();
        let ellipsis = highlighter.get_ellipsis();
        assert_eq!(ellipsis, "...");
    }

    #[test]
    fn test_get_ellipsis_custom() {
        let ellipsis_opt = HighlightEllipsisOptions {
            template: "[$1]".to_string(),
            pattern: Some("...".to_string()),
        };
        let options = HighlightOptions {
            template: "<b>$1</b>".to_string(),
            boundary: None,
            clip: None,
            merge: None,
            ellipsis: Some(ellipsis_opt),
        };
        let highlighter = Highlighter::new(&options).unwrap();
        let ellipsis = highlighter.get_ellipsis();
        assert_eq!(ellipsis, "[...]");
    }
}

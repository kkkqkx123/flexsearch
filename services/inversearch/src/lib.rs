pub mod charset;
pub mod common;
pub mod config;
pub mod encoder;
pub mod error;
pub mod highlight;
pub mod index;
pub mod intersect;
pub mod keystore;
pub mod metrics;
pub mod proto;
pub mod resolver;
pub mod search;
pub mod tokenizer;
pub mod r#type;

// Re-export charset modules with specific names to avoid conflicts
pub use charset::{
    charset_exact, charset_normalize, charset_cjk,
    charset_latin_balance, charset_latin_advanced, charset_latin_extra, charset_latin_soundex,
    get_charset_exact, get_charset_default, get_charset_normalize,
    get_charset_latin_balance, get_charset_latin_advanced, get_charset_latin_extra, get_charset_latin_soundex,
    get_charset_cjk, get_charset_latin_exact, get_charset_latin_default, get_charset_latin_simple,
    get_charset_polyfill, normalize_charset
};
pub use common::*;
pub use config::*;
pub use encoder::*;
pub use error::*;
pub use highlight::*;
pub use index::*;
pub use intersect::*;
pub use keystore::*;
pub use metrics::*;
pub use proto::*;
pub use resolver::*;
pub use search::*;
pub use tokenizer::*;
pub use r#type::*;

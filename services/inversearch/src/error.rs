use thiserror::Error;

#[derive(Debug, Error)]
pub enum InversearchError {
    #[error("Index error: {0}")]
    Index(#[from] IndexError),

    #[error("Search error: {0}")]
    Search(#[from] SearchError),

    #[error("Encoder error: {0}")]
    Encoder(#[from] EncoderError),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Cache error: {0}")]
    Cache(#[from] CacheError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

#[derive(Debug, Error)]
pub enum IndexError {
    #[error("Empty content")]
    EmptyContent,

    #[error("Invalid document ID: {0}")]
    InvalidId(u64),

    #[error("Document not found: {0}")]
    NotFound(u64),

    #[error("Encoding error: {0}")]
    Encoding(String),
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Empty query")]
    EmptyQuery,

    #[error("Invalid search options: {0}")]
    InvalidOptions(String),

    #[error("No results found")]
    NoResults,

    #[error("Search timeout")]
    Timeout,
}

#[derive(Debug, Error)]
pub enum EncoderError {
    #[error("Invalid regex: {0}")]
    InvalidRegex(String),

    #[error("Encoding error: {0}")]
    Encoding(String),

    #[error("Normalization error: {0}")]
    Normalization(String),
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Cache miss")]
    Miss,

    #[error("Cache error: {0}")]
    Error(String),
}

pub type Result<T> = std::result::Result<T, InversearchError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = IndexError::EmptyContent;
        assert_eq!(format!("{}", err), "Empty content");
    }

    #[test]
    fn test_search_error() {
        let err = SearchError::EmptyQuery;
        assert_eq!(format!("{}", err), "Empty query");
    }

    #[test]
    fn test_encoder_error() {
        let err = EncoderError::Encoding("test".to_string());
        assert_eq!(format!("{}", err), "Encoding error: test");
    }
}

//! Error handling for the rust2mojo compiler

use thiserror::Error;

/// Result type alias for the rust2mojo compiler
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for the rust2mojo compiler
#[derive(Error, Debug)]
pub enum Error {
    /// Rust parsing errors
    #[error("Failed to parse Rust code: {0}")]
    ParseError(String),
    
    /// Mojo code generation errors
    #[error("Failed to generate Mojo code: {0}")]
    CodegenError(String),
    
    /// I/O errors
    #[error("I/O error: {0}")]
    IoError(String),
    
    /// Unsupported Rust language feature
    #[error("Unsupported Rust feature: {0}")]
    UnsupportedFeature(String),
    
    /// Internal compiler error
    #[error("Internal compiler error: {0}")]
    InternalError(String),
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Error::ParseError(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err.to_string())
    }
}
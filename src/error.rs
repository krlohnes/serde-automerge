#[derive(Debug, PartialEq, Eq, Clone, thiserror::Error)]
pub enum Error {
    #[error("unsupported Rust type")]
    UnsupportedType,
    #[error("{0}")]
    Custom(String),
    #[error("expected boolean, found {0}")]
    ExpectedBoolean(&'static str),
    #[error("expected string, found {0}")]
    ExpectedString(&'static str),
    #[error("expected integer, found {0}")]
    ExpectedInteger(&'static str),
    #[error("expected float, found {0}")]
    ExpectedFloat(&'static str),
    #[error("expected null, found {0}")]
    ExpectedNull(&'static str),
    #[error("expected map, found {0}")]
    ExpectedMap(&'static str),
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

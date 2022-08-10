#[derive(Debug, PartialEq, Eq, Clone, thiserror::Error)]
pub enum Error {
    #[error("unsupported Rust type")]
    UnsupportedType,
    #[error("{0}")]
    Custom(String),
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

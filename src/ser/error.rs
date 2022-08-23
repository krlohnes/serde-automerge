#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),
    #[error("map keys must be a string")]
    KeysMustBeAString,
    #[error(transparent)]
    AutomergeError(#[from] automerge::AutomergeError),
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

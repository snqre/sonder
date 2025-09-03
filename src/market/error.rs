use super::*;

pub type Result<T> = ::std::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(::thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    QError(q::Error),
}

impl From<q::Error> for Error {
    fn from(value: q::Error) -> Self {
        Self::QError(value)
    }
}
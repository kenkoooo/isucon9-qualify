use actix_web::ResponseError;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Error {
    err: anyhow::Error,
}
impl ResponseError for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(e: E) -> Self {
        Self {
            err: anyhow::Error::from(e),
        }
    }
}

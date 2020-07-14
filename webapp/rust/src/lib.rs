pub mod constants;
pub mod error;
pub mod server;

pub type IsuResult<T> = std::result::Result<T, error::Error>;

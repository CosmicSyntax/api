use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum ApiErrors {
    ManagerError(&'static str),
}

pub const MANAGER_START_ERROR: ApiErrors = ApiErrors::ManagerError("could not start manager");

impl Error for ApiErrors {}

impl Display for ApiErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

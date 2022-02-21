use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum ApiErrors {
    ManagerError(&'static str),
}

pub const MANAGER_START_ERROR: ApiErrors = ApiErrors::ManagerError("Could not start manager.");

impl Error for ApiErrors {}

impl Display for ApiErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiErrors::ManagerError(m) => write!(f, "{}", m),
        }
    }
}

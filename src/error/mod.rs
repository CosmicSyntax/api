#[derive(Debug)]
pub enum ApiErrors<'a> {
    ManagerError(&'a str),
}

pub const MANAGER_START_ERROR: ApiErrors = ApiErrors::ManagerError("Could not start manager.");

#[derive(Debug)]
pub enum ApiErrors<'a> {
    ManagerError(&'a str),
}

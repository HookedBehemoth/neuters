pub enum ApiError {
    External(u16, String),
    InternalServerError(String),
}

pub enum ApiError {
    External(u16, String),
    Internal(String),
}

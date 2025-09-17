use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

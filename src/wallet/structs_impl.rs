use axum::{response::{Response, IntoResponse}, http::StatusCode,};

#[derive(serde::Serialize)]
pub struct AddressResponse{
    pub address: String,
    pub index: u32,
}

pub struct AppError(anyhow::Error);


impl<E> From<E> for AppError where E: Into<anyhow::Error>{
    fn from (err: E) -> Self{
        Self(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        ).into_response()
    }
}
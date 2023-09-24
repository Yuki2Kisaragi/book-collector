use crate::repositories::{BookRepository, CreateBook};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
pub async fn create_book<T: BookRepository>(
    Json(payload): Json<CreateBook>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let book = repository.create(payload);
    (StatusCode::CREATED, Json(book))
}

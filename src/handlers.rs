use crate::repositories::{BookRepository, CreateBook, UpdateBook};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
pub async fn create_book<T: BookRepository>(
    Json(payload): Json<CreateBook>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let book = repository.create(payload);
    (StatusCode::CREATED, Json(book))
}

pub async fn find_book<T: BookRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    todo!();
    Ok(StatusCode::OK)
}
pub async fn all_book<T: BookRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    todo!();
}
pub async fn update_book<T: BookRepository>(
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBook>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    todo!();
    Ok(StatusCode::OK)
}

pub async fn delete_book<T: BookRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    todo!();
}

use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::repositories::{BookRepository, CreateBook, UpdateBook};

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
    let book = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(book)))
}

pub async fn all_book<T: BookRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let book = repository.all();
    (StatusCode::OK, Json(book))
}

pub async fn update_book<T: BookRepository>(
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBook>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let book = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(book)))
}

pub async fn delete_book<T: BookRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}

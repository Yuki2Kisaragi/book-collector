use std::sync::Arc;

use axum::extract::{FromRequest, RequestParts};
use axum::{
    async_trait,
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;
use tower::BoxError;
use validator::Validate;

use crate::repositories::{BookRepository, CreateBook, UpdateBook};

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await.map_err(|rejection| {
            let message = format!("json parse error: [{}]", rejection);
            (StatusCode::BAD_REQUEST, message)
        })?;

        value.validate().map_err(|rejection| {
            let message = format!("Validation error: [{}]", rejection).replace('\n', ",");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

pub async fn create_book<T: BookRepository>(
    ValidatedJson(payload): ValidatedJson<CreateBook>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let book = repository
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(book)))
}

pub async fn find_book<T: BookRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let book = repository.find(id).await.or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(book)))
}

pub async fn all_book<T: BookRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let book = repository.all().await.unwrap();
    Ok((StatusCode::OK, Json(book)))
}

pub async fn update_book<T: BookRepository>(
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateBook>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let book = repository
        .update(id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(book)))
}

pub async fn delete_book<T: BookRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}

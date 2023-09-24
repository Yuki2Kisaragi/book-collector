use anyhow::Context;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};
use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

pub trait BookRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateBook) -> Book;
    fn find(&self, id: i32) -> Option<Book>;
    fn all(&self) -> Vec<Book>;
    fn update(&self, id: i32, payload: UpdateBook) -> anyhow::Result<Book>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Book {
    id: i32,
    name: String,
    isbn_code: String,
    author: String,
    revision_number: u32,
    publisher: String,
    // published_at: datetime
    // status: intEnum
    // created_at: datetime
    // updated_at:datetime
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct CreateBook {
    name: String,
    isbn_code: String,
    author: String,
    revision_number: u32,
    publisher: String,
    // published_at: datetime
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateBook {
    name: Option<String>,
    isbn_code: Option<String>,
    author: Option<String>,
    revision_number: Option<u32>,
    publisher: Option<String>,
    // published_at: datetime
}

impl Book {
    pub fn new(
        id: i32,
        name: String,
        isbn_code: String,
        author: String,
        revision_number: u32,
        publisher: String,
    ) -> Self {
        Self {
            id,
            name,
            isbn_code,
            author,
            revision_number,
            publisher,
        }
    }
}

type BookData = HashMap<i32, Book>;

#[derive(Debug, Clone)]
pub struct BookRepositoryForMemory {
    store: Arc<RwLock<BookData>>,
}

impl BookRepositoryForMemory {
    pub fn new() -> Self {
        BookRepositoryForMemory {
            store: Arc::default(),
        }
    }
}

impl BookRepository for BookRepositoryForMemory {
    fn create(&self, payload: CreateBook) -> Book {
        todo!()
    }
    fn find(&self, id: i32) -> Option<Book> {
        todo!()
    }
    fn all(&self) -> Vec<Book> {
        todo!()
    }
    fn update(&self, id: i32, payload: UpdateBook) -> anyhow::Result<Book> {
        todo!()
    }
    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!()
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct CreateUser {
    username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    username: String,
}

#[tokio::main]
async fn main() {
    // init logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = create_app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
}

async fn root() -> &'static str {
    "Hello World"
}

async fn create_user(
    // ここでデシリアライズ
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    // ここでシリアライズ
    (StatusCode::CREATED, Json(user))
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello World")
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "username": "MasaHero" }"#))
            .unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot convert User interface");

        assert_eq!(
            user,
            User {
                id: 1337,
                username: "MasaHero".to_string()
            }
        )
    }
}

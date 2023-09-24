mod handlers;
mod repositories;

use anyhow::Context;
use axum::{
    extract::Extension,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use handlers::create_book;
use repositories::{BookRepository, BookRepositoryForMemory};
use std::net::SocketAddr;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    // init logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let repository = BookRepositoryForMemory::new();
    let app = create_app(repository);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<T: BookRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/books", post(create_book::<T>))
        .layer(Extension(Arc::new(repository)))
}

async fn root() -> &'static str {
    "Hello World"
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = BookRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello World")
    }

    // #[tokio::test]
    // async fn should_return_user_data() {
    //     let repository = BookRepositoryForMemory::new();
    //     let req = Request::builder()
    //         .uri("/books")
    //         .method(Method::POST)
    //         .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
    //         .body(Body::from(
    //             r#"{ "name": "Rust Book",
    //                         "isbn_code": "ABCD1234",
    //                         "author": "MasaHero",
    //                         "revision_number": 1,
    //                         "publisher": "Rust Company",
    //          }"#))
    //         .unwrap();
    //     let res = create_app(repository).oneshot(req).await.unwrap();
    //     let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
    //     let body: String = String::from_utf8(bytes.to_vec()).unwrap();
    //     let user: User = serde_json::from_str(&body).expect("cannot convert User interface");
    //
    //     assert_eq!(
    //         user,
    //         User {
    //             id: 1337,
    //             name: "MasaHero".to_string()
    //         }
    //     )
    // }
}

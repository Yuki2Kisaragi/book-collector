use std::net::SocketAddr;
use std::{env, sync::Arc};

use anyhow::Context;
use axum::{
    extract::Extension,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use handlers::{all_book, create_book, delete_book, find_book, update_book};
use repositories::{BookRepository, BookRepositoryForMemory};

mod handlers;
mod repositories;

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
        .route("/books", post(create_book::<T>).get(all_book::<T>))
        .route(
            "/books/:id",
            get(find_book::<T>)
                .delete(delete_book::<T>)
                .patch(update_book::<T>),
        )
        .layer(Extension(Arc::new(repository)))
}

async fn root() -> &'static str {
    "Hello World"
}

#[cfg(test)]
mod test {
    use axum::http::StatusCode;
    use axum::response::Response;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    use crate::repositories::{Book, CreateBook};

    use super::*;

    async fn build_book_req_with_json(
        path: &str,
        method: Method,
        json_body: String,
    ) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json_body))
            .unwrap()
    }

    async fn build_book_req_with_empty(path: &str, method: Method) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .body(Body::empty())
            .unwrap()
    }

    async fn res_to_book(res: Response) -> Book {
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let book: Book = serde_json::from_str(&body)
            .expect(&format!("cannot convert Book instance. body: {}", body));
        book
    }

    #[tokio::test]
    async fn should_create_book() {
        let expected = Book::new(
            1,
            "created_book".to_string(),
            "isbn_code".to_string(),
            "author".to_string(),
            1,
            "publisher".to_string(),
        );

        let repository = BookRepositoryForMemory::new();
        let req = build_book_req_with_json(
            "/books",
            Method::POST,
            r#"{
            "id": 1,
            "name": "created_book",
            "isbn_code": "isbn_code",
            "author": "author",
            "revision_number": 1,
            "publisher": "publisher"
            }"#
            .to_string(),
        );
        let res = create_app(repository).oneshot(req.await).await.unwrap();
        let book = res_to_book(res).await;
        assert_eq!(expected, book);
    }

    #[tokio::test]
    async fn should_find_book() {
        let expected = Book::new(
            1,
            "created_book".to_string(),
            "isbn_code".to_string(),
            "author".to_string(),
            1,
            "publisher".to_string(),
        );
        let repository = BookRepositoryForMemory::new();
        repository
            .create(CreateBook::new(
                "created_book".to_string(),
                "isbn_code".to_string(),
                "author".to_string(),
                1,
                "publisher".to_string(),
            ))
            .await
            .expect("failed create book");
        let req = build_book_req_with_empty("/books/1", Method::GET);
        let res = create_app(repository).oneshot(req.await).await.unwrap();
        let book = res_to_book(res).await;
        assert_eq!(expected, book);
    }

    #[tokio::test]
    async fn should_get_all_books() {
        let expected = Book::new(
            1,
            "created_book".to_string(),
            "isbn_code".to_string(),
            "author".to_string(),
            1,
            "publisher".to_string(),
        );
        let repository = BookRepositoryForMemory::new();
        repository
            .create(CreateBook::new(
                "created_book".to_string(),
                "isbn_code".to_string(),
                "author".to_string(),
                1,
                "publisher".to_string(),
            ))
            .await
            .expect("failed create book");
        let req = build_book_req_with_empty("/books", Method::GET);
        let res = create_app(repository).oneshot(req.await).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let books: Vec<Book> = serde_json::from_str(&body)
            .expect(&format!("cannot convert Book instance. body: {}", body));
        assert_eq!(vec![expected], books);
    }

    #[tokio::test]
    async fn should_update_book() {
        let expected = Book::new(
            1,
            "updated_book".to_string(),
            "isbn_code2".to_string(),
            "author2".to_string(),
            2,
            "publisher2".to_string(),
        );
        let repository = BookRepositoryForMemory::new();
        repository
            .create(CreateBook::new(
                "created_book".to_string(),
                "isbn_code".to_string(),
                "author".to_string(),
                1,
                "publisher".to_string(),
            ))
            .await
            .expect("failed create book");
        let req = build_book_req_with_json(
            "/books/1",
            Method::PATCH,
            r#"{
            "name": "updated_book",
            "isbn_code": "isbn_code2",
            "author": "author2",
            "revision_number": 2,
            "publisher": "publisher2"
            }"#
            .to_string(),
        );
        let res = create_app(repository).oneshot(req.await).await.unwrap();
        let book = res_to_book(res).await;
        assert_eq!(expected, book);
    }

    #[tokio::test]
    async fn should_delete_book() {
        let repository = BookRepositoryForMemory::new();
        repository
            .create(CreateBook::new(
                "created_book".to_string(),
                "isbn_code".to_string(),
                "author".to_string(),
                1,
                "publisher".to_string(),
            ))
            .await
            .expect("failed create book");
        let req = build_book_req_with_empty("/books/1", Method::DELETE);
        let res = create_app(repository).oneshot(req.await).await.unwrap();
        assert_eq!(StatusCode::NO_CONTENT, res.status());
    }
}

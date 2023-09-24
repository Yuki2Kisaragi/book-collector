use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
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

use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use anyhow::Context;
use axum::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[async_trait]
pub trait BookRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateBook) -> anyhow::Result<Book>;
    async fn find(&self, id: i32) -> anyhow::Result<Book>;
    async fn all(&self) -> anyhow::Result<Vec<Book>>;
    async fn update(&self, id: i32, payload: UpdateBook) -> anyhow::Result<Book>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Validate)]
pub struct CreateBook {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over string length"))]
    name: String,

    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over string length"))]
    isbn_code: String,

    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over string length"))]
    author: String,

    #[validate(range(min = 1))]
    revision_number: u32,

    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over string length"))]
    publisher: String,
    // published_at: datetime
}

#[cfg(test)]
impl CreateBook {
    pub fn new(
        name: String,
        isbn_code: String,
        author: String,
        revision_number: u32,
        publisher: String,
    ) -> Self {
        Self {
            name,
            isbn_code,
            author,
            revision_number,
            publisher,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Validate)]
pub struct UpdateBook {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over string length"))]
    name: Option<String>,

    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over string length"))]
    isbn_code: Option<String>,

    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over string length"))]
    author: Option<String>,

    #[validate(range(min = 1))]
    revision_number: Option<u32>,

    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over string length"))]
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

    fn write_store_ref(&self) -> RwLockWriteGuard<BookData> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<BookData> {
        self.store.read().unwrap()
    }
}

#[async_trait]
impl BookRepository for BookRepositoryForMemory {
    async fn create(&self, payload: CreateBook) -> anyhow::Result<Book> {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let book = Book::new(
            id,
            payload.name.clone(),
            payload.isbn_code.clone(),
            payload.author.clone(),
            payload.revision_number.clone(),
            payload.publisher.clone(),
        );
        store.insert(id, book.clone());
        Ok(book)
    }

    async fn find(&self, id: i32) -> anyhow::Result<Book> {
        let store = self.read_store_ref();
        let book = store
            .get(&id)
            .map(|book| book.clone())
            .ok_or(RepositoryError::NotFound(id))?;
        Ok(book)
    }

    async fn all(&self) -> anyhow::Result<Vec<Book>> {
        let store = self.read_store_ref();
        Ok(Vec::from_iter(store.values().map(|book| book.clone())))
    }
    async fn update(&self, id: i32, payload: UpdateBook) -> anyhow::Result<Book> {
        let mut store = self.write_store_ref();
        let book = store.get(&id).context(RepositoryError::NotFound(id))?;
        let name = payload.name.unwrap_or(book.name.clone());
        let isbn_code = payload.isbn_code.unwrap_or(book.isbn_code.clone());
        let author = payload.author.unwrap_or(book.author.clone());
        let revision_number = payload
            .revision_number
            .unwrap_or(book.revision_number.clone());
        let publisher = payload.publisher.unwrap_or(book.publisher.clone());

        let book = Book {
            id,
            name,
            isbn_code,
            author,
            revision_number,
            publisher,
        };
        store.insert(id, book.clone());
        Ok(book)
    }
    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn todo_crud_scenario() {
        let id = 1;
        let name = "book_test".to_string();
        let isbn_code = "isbn_code_test".to_string();
        let author = "author_test".to_string();
        let revision_number = 1;
        let publisher = "publisher_test".to_string();

        let expected = Book::new(
            id.clone(),
            name.clone(),
            isbn_code.clone(),
            author.clone(),
            revision_number.clone(),
            publisher.clone(),
        );

        // create
        let repository = BookRepositoryForMemory::new();
        let book = repository
            .create(CreateBook {
                name,
                isbn_code,
                author,
                revision_number,
                publisher,
            })
            .await
            .expect("failed create book");
        assert_eq!(expected, book);

        // find
        let book = repository.find(book.id).await.unwrap();
        assert_eq!(expected, book);

        // all
        let books = repository.all().await.expect("failed get all books");
        assert_eq!(vec![expected], books);

        // update
        let updated_name = "book_test2".to_string();
        let updated_isbn_code = "isbn_code_test2".to_string();
        let updated_author = "author_test2".to_string();
        let updated_revision_number = 2;
        let updated_publisher = "publisher_test2".to_string();

        let updated_book = repository
            .update(
                id,
                UpdateBook {
                    name: Some(updated_name.clone()),
                    isbn_code: Some(updated_isbn_code.clone()),
                    author: Some(updated_author.clone()),
                    revision_number: Some(updated_revision_number.clone()),
                    publisher: Some(updated_publisher.clone()),
                },
            )
            .await
            .expect("failed update book");
        assert_eq!(repository.find(id).await.unwrap(), updated_book);

        // delete
        let res = repository.delete(id).await;
        assert!(res.is_ok());
    }
}

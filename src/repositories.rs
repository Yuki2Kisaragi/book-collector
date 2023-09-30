use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
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

    fn write_store_ref(&self) -> RwLockWriteGuard<BookData> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<BookData> {
        self.store.read().unwrap()
    }
}

impl BookRepository for BookRepositoryForMemory {
    fn create(&self, payload: CreateBook) -> Book {
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
        book
    }

    fn find(&self, id: i32) -> Option<Book> {
        let store = self.read_store_ref();
        store.get(&id).map(|book| book.clone())
    }

    fn all(&self) -> Vec<Book> {
        let store = self.read_store_ref();
        Vec::from_iter(store.values().map(|book| book.clone()))
    }
    fn update(&self, id: i32, payload: UpdateBook) -> anyhow::Result<Book> {
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
    fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn todo_crud_scenario() {
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
        let book = repository.create(CreateBook {
            name,
            isbn_code,
            author,
            revision_number,
            publisher,
        });
        assert_eq!(expected, book);

        // find
        assert_eq!(expected, repository.find(id).unwrap());

        // all
        let books = repository.all();
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
            .expect("failed update book");
        assert_eq!(repository.find(id).unwrap(), updated_book);

        // delete
        let res = repository.delete(id);
        assert!(res.is_ok());
    }
}

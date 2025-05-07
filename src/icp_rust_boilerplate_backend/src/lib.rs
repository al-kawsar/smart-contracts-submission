#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Type Definitions
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Struct Definitions
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Book {
    id: u64,
    title: String,
    author: String,
    genre: Genre,
    is_available: bool,
    borrower: Option<String>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default, PartialEq)]
enum Genre {
    #[default]
    Fiction,
    NonFiction,
    Science,
    Technology,
}

impl Storable for Book {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Book {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Thread-local Storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
        .expect("Cannot create a counter")
    );

    static BOOK_STORAGE: RefCell<StableBTreeMap<u64, Book, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        ));
}

#[derive(candid::CandidType, Serialize, Deserialize)]
struct BookPayload {
    title: String,
    author: String,
    genre: Genre,
}

#[ic_cdk::query]
fn get_book(id: u64) -> Result<Book, Error> {
    _get_book(&id).ok_or_else(|| Error::NotFound {
        msg: format!("Book with id={} not found", id),
    })
}

#[ic_cdk::query]
fn get_available_books() -> Vec<Book> {
    BOOK_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, book)| book.is_available)
            .map(|(_, book)| book.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn add_book(payload: BookPayload) -> Result<Book, Error> {
    validate_book_payload(&payload)?;

    let id = generate_unique_id()?;

    let book = Book {
        id,
        title: payload.title,
        author: payload.author,
        genre: payload.genre,
        is_available: true,
        borrower: None,
    };

    do_insert_book(&book);
    Ok(book)
}

#[ic_cdk::update]
fn borrow_book(book_id: u64) -> Result<Book, Error> {
    let mut book = _get_book(&book_id).ok_or_else(|| Error::NotFound {
        msg: format!("Book with id={} not found", book_id),
    })?;

    if !book.is_available {
        return Err(Error::InvalidOperation {
            msg: "Book is not available for borrowing".to_string(),
        });
    }

    book.is_available = false;
    book.borrower = Some(ic_cdk::caller().to_string());

    do_insert_book(&book);
    Ok(book)
}

#[ic_cdk::update]
fn return_book(book_id: u64) -> Result<Book, Error> {
    let mut book = _get_book(&book_id).ok_or_else(|| Error::NotFound {
        msg: format!("Book with id={} not found", book_id),
    })?;

    if book.is_available {
        return Err(Error::InvalidOperation {
            msg: "Book is already available".to_string(),
        });
    }

    book.is_available = true;
    book.borrower = None;

    do_insert_book(&book);
    Ok(book)
}

#[ic_cdk::update]
fn delete_book(id: u64) -> Result<(), Error> {
    _get_book(&id).ok_or_else(|| Error::NotFound {
        msg: format!("Book with id={} not found", id),
    })?;

    BOOK_STORAGE.with(|service| service.borrow_mut().remove(&id));
    Ok(())
}

fn _get_book(id: &u64) -> Option<Book> {
    BOOK_STORAGE.with(|service| service.borrow().get(id))
}

fn do_insert_book(book: &Book) {
    BOOK_STORAGE.with(|service| service.borrow_mut().insert(book.id, book.clone()));
}

fn generate_unique_id() -> Result<u64, Error> {
    ID_COUNTER.with(|counter: &RefCell<IdCell>| {
        let mut borrowed_counter = counter.borrow_mut();
        let current_value = *borrowed_counter.get();
        borrowed_counter.set(current_value + 1).map_err(|_| Error::InternalError {
            msg: "Failed to set new ID in counter".to_string(),
        })?;
        Ok(current_value + 1)
    })
}


fn validate_book_payload(payload: &BookPayload) -> Result<(), Error> {
    if payload.title.trim().is_empty() {
        return Err(Error::InvalidInput {
            msg: "Book title cannot be empty".to_string(),
        });
    }

    if payload.author.trim().is_empty() {
        return Err(Error::InvalidInput {
            msg: "Book author cannot be empty".to_string(),
        });
    }

    Ok(())
}

// Additional Functions
#[ic_cdk::query]
fn search_books_by_genre(genre: Genre) -> Vec<Book> {
    BOOK_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, book)| book.genre == genre)
            .map(|(_, book)| book.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn list_all_books() -> Vec<Book> {
    BOOK_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, book)| book.clone())
            .collect()
    })
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidOperation { msg: String },
    InvalidInput { msg: String },
    InternalError { msg: String },
}

ic_cdk::export_candid!();

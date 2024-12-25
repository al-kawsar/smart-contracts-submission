#[macro_use]
extern crate serde;

use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Memory management types
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Struct representing a book in the library system
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Book {
    id: u64,
    title: String,
    author: String,
    genre: Genre,
    is_available: bool,
    borrower: Option<String>,
}

// Enum representing the genre of a book
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default, PartialEq)]
enum Genre {
    #[default]
    Fiction,
    NonFiction,
    Science,
    Technology,
}

// Implement `Storable` for Book for stable storage encoding/decoding
impl Storable for Book {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement `BoundedStorable` for size and fixed size definition
impl BoundedStorable for Book {
    const MAX_SIZE: u32 = 1024; // Maximum allowed size for a book
    const IS_FIXED_SIZE: bool = false;
}

// Thread-local stable storage and ID counter
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static BOOK_STORAGE: RefCell<StableBTreeMap<u64, Book, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );
}

// Payload for creating a new book
#[derive(candid::CandidType, Serialize, Deserialize)]
struct BookPayload {
    title: String,
    author: String,
    genre: Genre,
}

// Query: Retrieve a book by ID
#[ic_cdk::query]
fn get_book(id: u64) -> Result<Book, Error> {
    match _get_book(&id) {
        Some(book) => Ok(book),
        None => Err(Error::NotFound {
            msg: format!("Book with ID={} not found", id),
        }),
    }
}

// Query: Retrieve all available books
#[ic_cdk::query]
fn get_available_books() -> Vec<Book> {
    BOOK_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .filter(|(_, book)| book.is_available)
            .map(|(_, book)| book.clone())
            .collect()
    })
}

// Update: Add a new book to the library
#[ic_cdk::update]
fn add_book(payload: BookPayload) -> Result<Book, Error> {
    // Validate input
    if payload.title.trim().is_empty() || payload.author.trim().is_empty() {
        return Err(Error::InvalidOperation {
            msg: "Title and author cannot be empty.".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

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

// Update: Borrow a book by ID
#[ic_cdk::update]
fn borrow_book(book_id: u64) -> Result<Book, Error> {
    match _get_book(&book_id) {
        Some(mut book) => {
            if !book.is_available {
                return Err(Error::InvalidOperation {
                    msg: "Book is not available for borrowing.".to_string(),
                });
            }

            book.is_available = false;
            book.borrower = Some(ic_cdk::caller().to_string());
            do_insert_book(&book);
            Ok(book)
        }
        None => Err(Error::NotFound {
            msg: format!("Book with ID={} not found.", book_id),
        }),
    }
}

// Update: Return a borrowed book
#[ic_cdk::update]
fn return_book(book_id: u64) -> Result<Book, Error> {
    match _get_book(&book_id) {
        Some(mut book) => {
            if book.is_available {
                return Err(Error::InvalidOperation {
                    msg: "Book is already available.".to_string(),
                });
            }

            book.is_available = true;
            book.borrower = None;
            do_insert_book(&book);
            Ok(book)
        }
        None => Err(Error::NotFound {
            msg: format!("Book with ID={} not found.", book_id),
        }),
    }
}

// Update: Delete a book by ID
#[ic_cdk::update]
fn delete_book(id: u64) -> Result<(), Error> {
    match _get_book(&id) {
        Some(_) => {
            BOOK_STORAGE.with(|storage| storage.borrow_mut().remove(&id));
            Ok(())
        }
        None => Err(Error::NotFound {
            msg: format!("Book with ID={} not found.", id),
        }),
    }
}

// Helper: Retrieve a book by ID
fn _get_book(id: &u64) -> Option<Book> {
    BOOK_STORAGE.with(|storage| storage.borrow().get(id))
}

// Helper: Insert or update a book in storage
fn do_insert_book(book: &Book) {
    BOOK_STORAGE.with(|storage| storage.borrow_mut().insert(book.id, book.clone()));
}

// Error handling enum
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidOperation { msg: String },
}

// Export candid interface
ic_cdk::export_candid!();

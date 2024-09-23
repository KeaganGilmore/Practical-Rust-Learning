use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Define a trait `Summarizable` with a method `summary`
trait Summarizable {
    fn summary(&self) -> String;
}

// Implement the `Summarizable` trait for the `Book` struct
struct Book {
    title: String,
    author: String,
    content: String,
}

impl Summarizable for Book {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// Implement the `Summarizable` trait for the `Article` struct
struct Article {
    headline: String,
    location: String,
    content: String,
}

impl Summarizable for Article {
    fn summary(&self) -> String {
        format!("{} in {}", self.headline, self.location)
    }
}

// Define a generic function that can summarize any type that implements `Summarizable`
fn print_summary<T: Summarizable>(item: &T) {
    println!("Summary: {}", item.summary());
}

// Define a function that demonstrates ownership and borrowing
fn ownership_and_borrowing() {
    let book = Book {
        title: String::from("Rust Programming"),
        author: String::from("John Doe"),
        content: String::from("Rust is a systems programming language..."),
    };

    // Borrowing the book to print its summary
    print_summary(&book);

    // Ownership of `book` is moved to `book_summary`
    let book_summary = book.summary();

    // `book` can no longer be used here as its ownership has been moved
    println!("Book summary: {}", book_summary);
}

// Define a function that demonstrates concurrency with threads
fn concurrency_example() {
    // Create a shared, mutable HashMap protected by a Mutex and wrapped in an Arc
    let data = Arc::new(Mutex::new(HashMap::new()));

    // Create a vector to hold the handles of spawned threads
    let mut handles = vec![];

    for i in 0..10 {
        // Clone the Arc to get a new reference to the shared data
        let data = Arc::clone(&data);

        // Spawn a new thread
        let handle = thread::spawn(move || {
            // Lock the Mutex to get mutable access to the data
            let mut data = data.lock().unwrap();
            // Insert a value into the HashMap
            data.insert(i, i * 10);
        });

        // Store the thread handle
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Lock the Mutex to get read access to the data
    let data = data.lock().unwrap();
    // Print the contents of the HashMap
    println!("Final data: {:?}", *data);
}

fn main() {
    // Demonstrate ownership and borrowing
    ownership_and_borrowing();

    // Demonstrate concurrency with threads
    concurrency_example();
}
use std::{fs::File, io::ErrorKind};

fn main() {
    // Recoverable Errors with Result

    // Most errors aren’t serious enough to require the program to stop entirely.
    // Sometimes when a function fails, it’s for a reason that you can easily interpret and respond to.

    // Result enum is defined as having two variants, Ok and Err, as follows:
    //  enum Result<T, E> {
    //      Ok(T),
    //      Err(E),
    //  }

    // we can use the Result type and the functions defined on it in many different situations where
    // the success value and error value we want to return may differ.

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match &greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {error:?}"),
    };

    // Note that, like the Option enum, the Result enum and its variants have been brought into scope by the prelude,
    // so we don’t need to specify Result:: before the Ok and Err variants in the match arms.

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file {error:?}"),
            },
            _ => panic!("Problem opening the file {error:?}"),
        },
    };

    // `File::open` returns a `Result<T, E>` where the `Err` variant contains an `io::Error`.
    // `io::Error` is a struct provided by the standard library.
    // It has a `kind()` method that returns an `io::ErrorKind` value.
    // `io::ErrorKind` is an enum defined in the standard library.
    // Its variants represent different kinds of I/O errors that can occur.

    // Using `match` with `Result<T, E>` can become verbose.
    // While `match` is powerful, it is a low-level primitive.
    // Rust provides many helper methods on `Result<T, E>`.
    // These methods are often used with closures.
    // They allow more concise and readable error handling.
    // Closures and these methods are covered in Chapter 13.

    let greeting_file = File::open("hello_closure.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello_closure.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {error:?}");
            })
        } else {
            panic!("Problem opening the file : {error:?}");
        }
    });

    //
    // Shortcuts for Panic on Error

    // `unwrap` is a shortcut method equivalent to a `match` on `Result`.
    // If the value is `Ok`, `unwrap` returns the inner value.
    // If the value is `Err`, `unwrap` calls the `panic!` macro.
    // This behavior matches the manual `match` logic shown earlier.

    // let greeting_file: File = File::open("hello_unwrap.txt").unwrap();

    // `expect` works like `unwrap` but lets you specify a custom panic message.
    // Providing a clear error message helps express intent.
    // It also makes debugging panics much easier.

    let greeting_file: File = File::open("hello_expect.txt")
        .expect("hello_expect.txt should be included in this project");

    // In production-quality code, Rustaceans usually prefer `expect` over `unwrap`.
    // A descriptive message explains why the operation is expected to succeed.
    // This adds clarity and improves debugging when a panic occurs.
}

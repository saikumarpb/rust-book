use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read},
};

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

    // let greeting_file: File = File::open("hello_expect.txt")
    //     .expect("hello_expect.txt should be included in this project");

    // In production-quality code, Rustaceans usually prefer `expect` over `unwrap`.
    // A descriptive message explains why the operation is expected to succeed.
    // This adds clarity and improves debugging when a panic occurs.

    //
    // Propagating Errors

    // When a function calls an operation that may fail,
    // it doesn’t have to handle the error itself.
    // Instead, it can return the error to the caller.
    // This is called **error propagation**.
    // It gives the calling code more control over how to handle the error,
    // since the caller may have more context or decision-making logic.

    let result: Result<String, Error> = read_username_from_file();
    // let username = result.expect("Username file not found");
    let username = result.unwrap();

    println!("Username from file : {username}");

    // The ? Operator Shortcut

    // The `?` operator works similarly to a `match` on `Result`.
    // If the value is `Ok`, the inner value is extracted and execution continues.
    // If the value is `Err`, the function returns early with the error.
    // propagating it to the caller as if `return Err(e)` were written.
    //
    // Unlike a manual `match`, the `?` operator also performs error conversion.
    // It uses the `from` function from the `From` trait in the standard library.
    // This converts the error type into the function’s declared return error type.
    //
    // This allows a function to return a single error type
    // even when different operations may fail for different reasons.
    //
    // For example, a function can return a custom `OurError` type.
    // By implementing `From<io::Error> for OurError`,
    // any `io::Error` encountered with `?` is automatically converted.
    // This keeps the function body clean without extra error-handling code.

    let result = read_username_from_file_question_mark_operator_v1();
    println!("read_username_from_file_question_mark_operator_v1 : {username}");

    let result = read_username_from_file_question_mark_operator_v2();
    println!("read_username_from_file_question_mark_operator_v2 : {username}");

    let result = read_username_from_file_question_mark_operator_v3();
    println!("read_username_from_file_question_mark_operator_v3 : {username}");

    // Where to Use the ? Operator

    // The `?` operator can only be used in functions whose return type
    // is compatible with the value the `?` is applied to.
    // This is because `?` performs an early return from the function.
    // It works the same way as the `match` expression shown earlier.
    // In that example, the early return returned `Err(e)`.
    // Therefore, the function’s return type must be `Result`
    // so it can return an error value in that form.

    // let greeting_file = File::open("hello.txt")?;

    // This ABOVE LINE of code attempts to open a file, which may fail.
    // The `?` operator is applied to the `Result` returned by `File::open`.
    // However, the `main` function returns `()` instead of `Result`.
    // Because of this mismatch, the code does not compile.

    // This error points out that we’re only allowed to use the `?` operator
    // in a function that returns `Result`, `Option`, or another type that
    // implements `FromResidual`.

    // To fix this error, you have two choices:
    //
    // 1️. Change the return type of your function to be compatible with
    //    the value you’re using the `?` operator on, as long as there are
    //    no restrictions preventing that.
    //
    // 2️. Use a `match` expression or one of the `Result<T, E>` methods
    //    to handle the `Result<T, E>` in whatever way is appropriate.

    // The error message also mentions that the `?` operator can be used
    // with `Option<T>` values as well.

    // Just like `Result`, you can only use `?` on an `Option` inside a
    // function that returns an `Option`.

    // The behavior of the `?` operator on `Option<T>` is similar to its
    // behavior on `Result<T, E>`:
    //
    // - If the value is `None`, the function returns `None` early.
    // - If the value is `Some`, the value inside `Some` becomes the result
    //   of the expression and the function continues executing.

    // Here's an example of a function that finds the last
    // character of the first line in the given text.
    let last_char = last_char_of_first_line("kfg\nk\njsflsg").expect("There's no first line");
    println!("Last char : {last_char}"); // prints g

    let last_char = last_char_of_first_line("\nk\njsflsg").expect("There's no first line"); // gets errrored because there's no first line
}

fn read_username_from_file() -> Result<String, Error> {
    let username_file_result: Result<File, Error> = File::open("hello_username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

fn read_username_from_file_question_mark_operator_v1() -> Result<String, Error> {
    let mut username_file = File::open("hello_username.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_question_mark_operator_v2() -> Result<String, Error> {
    let mut username = String::new();
    File::open("hello_username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_question_mark_operator_v3() -> Result<String, Error> {
    // Reading a file into a string is a common operation.
    // The standard library provides `fs::read_to_string` for this purpose.
    // It opens the file, creates a new `String`,
    // reads the file contents into the string,
    // and returns the populated `String`.

    fs::read_to_string("hello_username.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    // This function returns `Option<char>` because it’s possible that
    // there is a character there, but it’s also possible that there isn’t.

    // This code takes the `text` string slice argument and calls the
    // `lines` method on it, which returns an iterator over the lines
    // in the string.

    // Because the function wants to examine the first line, it calls
    // `next` on the iterator to get the first value.

    // If `text` is an empty string, calling `next` will return `None`.
    // In that case, we use `?` to stop execution early and return `None`
    // from `last_char_of_first_line`.

    // If `text` is not empty, `next` returns a `Some` value containing
    // a string slice of the first line in `text`.
}

use std::net::IpAddr;

use ch09_03_to_panic_or_not_to_panic::guessing_game::Guess;

fn main() {
    // So how do you decide when to use `panic!` and when to return `Result`?

    // When code panics, there is no way to recover.
    // The program stops immediately.

    // You *could* call `panic!` for any kind of error,
    // whether recovery is possible or not.

    // But doing that means **you are deciding** that the error
    // is unrecoverable on behalf of the calling code.

    // When you return a `Result`, you give control to the caller.

    // The calling code can choose to:
    // - try to recover from the error, or
    // - decide the error is unrecoverable and call `panic!` itself.

    // This flexibility is important because the caller often has
    // more context about what should happen next.

    // Because of this, returning `Result` is usually the best default
    // choice when you’re writing a function that might fail.

    //
    //
    // Examples, Prototype Code, and Tests

    // When writing examples to explain a concept,
    // adding full error-handling can make the example harder to understand.

    // In examples, it’s understood that calls like `unwrap()`
    // (which may panic) are just placeholders.

    // They represent where proper error handling would exist
    // in real application code, depending on the situation.

    // Similarly, `unwrap()` and `expect()` are very useful
    // during prototyping.

    // At that stage, you may not yet know how errors should be handled.

    // Using `unwrap()` or `expect()` leaves clear markers in the code
    // showing places that need better error handling later.

    // In tests, the situation is different.

    // If a method call fails inside a test,
    // you usually want the entire test to fail immediately.

    // Since `panic!` is how Rust marks a test as failed,
    // calling `unwrap()` or `expect()` is exactly the correct behavior.

    // For this reason, using `unwrap()` and `expect()`
    // is completely acceptable and idiomatic in tests.

    //
    //
    // When you have more information than the compiler, it can be acceptable
    // to call `expect()`. Sometimes your program contains logic that guarantees
    // a Result will be Ok, even though the compiler cannot understand or verify
    // that guarantee on its own.

    // From Rust’s point of view, the operation can still fail in general,
    // so you must handle the Result. However, if you can manually reason
    // that an Err value is impossible in your specific situation,
    // using `expect()` is reasonable.

    // In these cases, the message passed to `expect()` should clearly explain
    // why the failure cannot happen, serving both as documentation
    // and as a helpful error message if the assumption is ever violated.

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    //
    //
    // Guidelines for error handling:
    //
    // Panic when your program reaches a bad state — when assumptions,
    // guarantees, or invariants are broken due to invalid or missing values.
    // These situations are unexpected and usually indicate bugs.

    // Use panic! when continuing execution could be unsafe, insecure,
    // or when the program cannot reasonably recover.

    // Return Result when failure is expected and normal,
    // such as invalid user input, parsing errors, or network failures.
    // This allows the calling code to decide how to handle the error.

    // Panic is also appropriate when function contracts are violated.
    // Contract violations always indicate programmer mistakes,
    // not recoverable runtime errors.

    // Rust’s type system helps avoid many runtime checks.
    // Using types like Option, Result, or u32 lets the compiler
    // enforce correctness before the program runs.

    //
    //
    // Custom Types for Validation
    // let guess: Guess = Guess::new(1000); // panics as it validates the value should be between 1 to 100

    let guess: Guess = Guess::new(12);

    let secret_number = 66; // hardcoded here just for demonstrating example
    if guess.value() > secret_number {
        println!("Too high!");
    } else if guess.value() < secret_number {
        println!("Too low!");
    } else {
        println!("You win!");
    }

    // Rust’s error-handling features help you write more robust code.
    //
    // The panic! macro indicates that the program has reached a state
    // it cannot safely handle, so execution should stop immediately
    // rather than continuing with invalid or incorrect values.
    //
    // The Result enum represents operations that may fail in a
    // recoverable way. It uses Rust’s type system to force the caller
    // to handle both success and failure cases.
    //
    // Using panic! for unrecoverable errors and Result for recoverable
    // ones makes your code safer, clearer, and more reliable.
}

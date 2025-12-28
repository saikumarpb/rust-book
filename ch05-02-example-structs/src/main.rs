#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle1 = Rectangle {
        width: 39,
        height: 10,
    };

    // Use &Struct in function parameters to borrow a struct instead of taking ownership.

    println!("{}", area(&rectangle1));
    println!("{}", rectangle1.width);

    // Adding Functionality with Derived Traits

    // The primitive types we’ve seen so far implement Display by default because
    // there’s only one way you’d want to show a 1 or any other primitive type to a user.

    // structs don’t have a provided implementation of Display to use with println! and the {} placeholder.

    // Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug.
    // The Debug trait enables us to print our struct in a way that is useful for developers so that we can see its value while we’re debugging our code.

    // Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct.
    // To do that, we add the outer attribute #[derive(Debug)] just before the struct definition

    println!("rect1 is {rectangle1:?}");

    // When we have larger structs, it’s useful to have output that’s a bit easier to read;
    // in those cases, we can use {:#?} instead of {:?} in the println! string.
    println!("{rectangle1:#?}");

    // Another way to print out a value using the Debug format is to use the dbg! macro, which takes ownership of an expression
    // (as opposed to println!, which takes a reference), prints the file and line number of where that dbg! macro call occurs in
    // your code along with the resultant value of that expression, and returns ownership of the value.
    dbg!(&rectangle1);

    // Here’s an example where we’re interested in the value that gets assigned to the width field,
    // as well as the value of the whole struct in rect1:
    let scale = 3;
    let rectangle2 = Rectangle {
        width: dbg!(scale * 20), // dbg! returns ownership of the expression’s value
        height: 30,
    };

    // We don’t want dbg! to take ownership of rectangle2, so we use a reference to rectangle2
    dbg!(&rectangle2);

    println!("{rectangle2:#?}");
    // Note: Calling the dbg! macro prints to the standard error console stream (stderr),
    // as opposed to println!, which prints to the standard output console stream (stdout).
    // We’ll talk more about stderr and stdout in the “Redirecting Errors to Standard Error” section in Chapter 12.

    // In addition to the Debug trait, Rust has provided a number of traits for us to use with the derive attribute that can add useful behavior to our custom types. 
}

//  we want to borrow the struct rather than take ownership of it.
// This way, main retains its ownership and can continue using rectangle1,
// which is the reason we use the & in the function signature and where we call the function.

fn area(rectangle: &Rectangle) -> u32 {
    // accessing fields of a borrowed struct instance does not move the field values
    rectangle.width * rectangle.height
}

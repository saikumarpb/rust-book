fn main() {
    // Rust code uses snake case as the conventional style for function and variable names
    // Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
    // In function signatures, type of each parameter must be decalred.
    another_function(4);
    print_labelled_measurement(5, 'V');

    // Statements and Expressions
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value.

    // Function definitions are also statements;
    // Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression
    // Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // Functions with Return Values
    // In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
    // You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(8);
    println!("The value of z is: {z}");

    // Functions without return values return empty return type called a unit type (). This unit type is a tuple without any values.
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32) {
    println!("Another function {x}!");
}

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

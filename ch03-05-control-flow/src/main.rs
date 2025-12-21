fn main() {
    let number = 11;

    // if Expressions

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // Using if in a let Statement
    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable,

    let num2 = if number > 10 { number } else { 10 };

    println!("The value of num2 is: {num2}");

    // Loops
    // Rust has three kinds of loops: loop, while, and for.

    let mut num = 0;
    loop {
        num = num + 1;
        println!("{num} Again!");
        if num >= 20 {
            break;
        }
    }

    // Returning Values from Loops
    // The value to be returned can be specified after the break expression, and that value is returned from the loop and can be used outside it.
    // A return statement can also be used inside a loop. While break exits only the current loop, return exits the entire function.

    let mut counter = 0;
    let number_x_2 = loop {
        counter += 1;

        if counter == number {
            break number * 2;
        }
    };

    println!("The value of number x 2 is {number_x_2}");

    // When loops are nested, break and continue apply to the innermost loop by default.
    // A loop label can optionally be specified and used with break or continue to target a specific loop instead.
    // Loop labels must begin with a single quote.

    // While loop

    let mut num = 4;
    while num > 0 {
        num -= 1;
        println!("num : {num}");
    }

    // For loop
    let a = [10, 20, 30, 40, 50];

    for ele in a {
        println!("The value in a is : {ele}");
    }

    for num in (1..4).rev() {
        println!("Countdown: {num}");
    }
    println!("Let's Play!");
}

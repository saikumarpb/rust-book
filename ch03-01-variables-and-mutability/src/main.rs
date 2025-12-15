fn main() {
    // Variables and Mutability

    // variables are immutable by default, you can make them mutable by adding mut in front of the variable name
    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;
    println!("The value of x is {x}");

    // Declaring Constants

    /**
     * Constants aren’t just immutable by default—they’re always immutable. You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.
     * Rust’s naming convention for constants is to use all uppercase with underscores between words.
     * Naming hardcoded values used throughout your program as constants is useful in conveying the meaning of that value to future maintainers of the code.
     */

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 6 * 3;

    // Shadowing

    // Difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
    // `mut` allows you to change the value of a variable, but the data type must remain the same. Assigning a value of a different type causes a compilation error.
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // Example for :we can change the type of the value but reuse the same name.

    // The first spaces variable is a string type, and the second spaces variable is a number type.
    // Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num;

    let spaces = "    ";
    let spaces = spaces.len();
}

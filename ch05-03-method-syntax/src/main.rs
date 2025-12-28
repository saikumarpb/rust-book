#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Everything within this impl block will be associated with the Rectangle type.
impl Rectangle {
    // In the signature for area, we use &self instead of rectangle: &Rectangle.
    // The &self is actually short for self: &Self

    // Within an impl block, the type Self is an alias for the type that the impl block is for.
    // Methods must have a parameter named self of type Self for their first parameter,
    // so Rust lets you abbreviate this with only the name self in the first parameter spot.
    // Note that we still need to use the & in front of the self shorthand to indicate that this method borrows the Self instance,
    // just as we did in rectangle: &Rectangle.
    // Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably,
    // just as they can any other parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Use `&mut self` when the method needs to modify the instance.
    fn double_the_width(&mut self) {
        self.width = 2 * self.width;
    }

    //  Having a method that takes ownership of the instance by using just self as the first parameter is rare;
    // this technique is usually used when the method transforms self into something else and
    // you want to prevent the caller from using the original instance after the transformation.
    fn double_the_width_with_move(mut self) {
        self.width = 2 * self.width;
    }

    // Note that we can choose to give a method the same name as one of the struct’s fields.
    fn width(&self) -> bool {
        self.width > 0
    }

    // Methods can take multiple parameters that we add to the signature after the self parameter, and
    // those parameters work just like parameters in functions.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // We can define associated functions that don’t have self as their first parameter (and thus are not methods) because
    // they don’t need an instance of the type to work with.
    // To call this associated function, we use the :: syntax with the struct name;
    // This function is namespaced by the struct.
    fn square(length: u32) -> Self {
        Self {
            width: length,
            height: length,
        }
    }
}

// Each struct is allowed to have multiple impl blocks
impl Rectangle {
    fn print(&self) {
        println!("Width: {}, Height: {}", self.width, self.height);
    }
}

fn main() {
    // Methods are similar to functions:
    // We declare them with the fn keyword and a name, they can have parameters and a return value,
    // and they contain some code that’s run when the method is called from somewhere else.

    // Unlike functions, methods are defined within the context of a struct (or an enum or a trait object)
    // and their first parameter is always self,
    // which represents the instance of the struct the method is being called on.

    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    println!(
        "The area of rectangle rect1 (Width: {}, Height: {}) is {}",
        rect1.width,
        rect1.height,
        rect1.area()
    );

    let mut rect2 = Rectangle {
        width: 10,
        height: 10,
    };

    rect2.double_the_width();

    println!(
        "The area of rectangle rect2 (Width: {}, Height: {}) is {}",
        rect2.width,
        rect2.height,
        rect2.area()
    );

    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }

    // In Rust, methods can have the same name as struct fields, commonly for getters.
    // Getters simply return the field’s value without extra logic. Rust doesn’t generate
    // getters automatically, so we define them when needed. They’re especially useful
    // for keeping fields private while exposing controlled, read-only access through
    // a public method in the struct’s API. more on this in chapter 7

    // Rust has a feature called automatic referencing and dereferencing.
    // Calling methods is one of the few places in Rust with this behavior.
    // When you call a method with object.something(), Rust automatically adds in &, &mut, or * so that object matches the signature of the method.
    // In other words, the following are the same:
    rect1.can_hold(&rect2); // looks much cleaner
    (&rect1).can_hold(&rect2);

    // The fact that Rust makes borrowing implicit for
    // method receivers is a big part of making ownership ergonomic in practice.

    // The :: syntax is used for both associated functions and namespaces created by modules.
    // We’ll discuss modules in Chapter 7.
    let sqr1 = Rectangle::square(3);
    println!("Area of sqr1: {}", sqr1.area());

    sqr1.print();

    // Structs allow you to model meaningful domain types by grouping related data with
    // clearly named fields, improving code clarity. `impl` blocks let you define functions
    // tied to that type, known as associated functions, while methods (a special kind of
    // associated function) define behaviors that operate on struct instances via `self`.
    // Structs aren’t the only way to create custom types in Rust—enums provide another
    // powerful option for modeling variant-based data and behavior.
}

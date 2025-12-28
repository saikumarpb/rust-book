struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct Cordinates(i32, i32, i32);

fn main() {
    // Defining and Instantiating Structs

    // Structs are similar to tuples. both hold multiple related values.
    // Like tuples, the pieces of a struct can be different types.
    // Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean.
    // Adding these names means that structs are more flexible than tuples:
    // You don’t have to rely on the order of the data to specify or access the values of an instance.

    // To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields

    let mut user1 = User {
        username: String::from("skiier_sol"),
        email: String::from("lets@get.rusty"),
        active: true,
        sign_in_count: 10,
    };

    // To get a specific value from a struct, we use dot notation.
    // If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.
    // Rust doesn’t allow us to mark only certain fields as mutable.
    user1.email = String::from("newbie@get.rusty");

    let user2 = build_user(String::from("user2"), String::from("user2@x.com"));

    // Creating Instances with Struct Update Syntax
    // Using struct update syntax,The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

    let user3 = User {
        email: String::from("user3@x.com"),
        username: String::from("user3"),
        ..user1
    };

    // ALl fields of user1 would still be valid here after creating user3.
    // Both active and sign_in_count are types that implement the Copy trait,
    // so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.

    let user4 = User {
        email: String::from("user4@x.com"),
        ..user1
    };

    // user1.username is invalid here because the String in username field of user1 was moved into user4.
    // user1.email is valid as its not moved into user4
    // struct update syntax uses = like an assignment; this is because it moves the data, just as we saw in the “Variables and Data Interacting with Move” section.

    // println!("{0}", user1.username); // Produces compile error "Borrow of moved value"

    // Creating Different Types with Tuple Structs

    // Rust also supports structs that look similar to tuples, called tuple structs.
    // Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.
    let black = Color(0, 0, 0);
    let origin = Cordinates(0, 0, 0);

    // Accessing tuple struct values
    // Unlike tuples, tuple structs require you to name the type of the struct when you destructure them.
    let Cordinates(x, y, z) = origin;
    println!("{x} {y} {z} {0}", black.0);

    // Destructuring tuples
    let tuple = (2, 3, 4);
    let (a, b, c) = tuple;
    println!("{a} {b} {c}");

    // Note that the black and origin values are different types because they’re instances of different tuple structs.
    // Each struct you define is its own type, even though the fields within the struct might have the same types.
    // For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values.

    // Defining Unit-Like Structs

    // You can also define structs that don’t have any fields!
    // These are called unit-like structs because they behave similarly to (), the unit type
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // Unit-like structs can be useful when you need to implement
    // a trait on some type but don’t have any data that you want to store in the type itself.
    // More on this in upcoming chapters

    //Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type,
    // perhaps to have a known result for testing purposes. We wouldn’t need any data to implement that behavior!

    //
    // Ownership of Struct Data

    // In the User struct definition, we used the owned String type rather than the &str string slice type.
    // This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid
    // for as long as the entire struct is valid.

    // In structs, use String (owned type) when:
    // You want the struct to own its data
    // The data should live as long as the struct exists

    // &str is a reference (borrowed type):
    // It does not own data
    // The real text is stored somewhere else

    // Structs can store references like &str, but:
    // You must specify lifetimes (explained in Chapter 10)
    // Lifetimes guarantee the referenced data stays valid while the struct needs it

    // If you store a reference in a struct without lifetimes, Rust will throw an error ❌
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

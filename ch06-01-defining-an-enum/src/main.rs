#[derive(Debug)]
struct Ipv4Addr {}

#[derive(Debug)]
struct Ipv6Addr {}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8), // variants of the enum are namespaced under its identifier.
    V6(String),
    V4Struct(Ipv4Addr),
    V6Struct(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,                    // Has no data associated with it at all
    Move { x: i32, y: i32 }, // Has named fields, like a struct does
    Write(String),           // Includes a single String
    ChangeColor(u8, u8, u8), // Includes three i32 values
}

// Defining an enum with variants like above is similar to defining multiple struct types.
// The difference is enums don’t use the `struct` keyword.
// Also, all enum variants are grouped under a single type (`Message`).
// The following struct definitions can hold the same data as the enum variants above.

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(u8, u8, u8);

// If we used different structs, each would have its own unique type.
// That means a function cannot accept all of them easily as parameters.
// But the `Message` enum is a single type, so one function can accept any variant.
// This makes enums more flexible than multiple independent structs.

// Just as we’re able to define methods on structs using impl, we’re also able to define methods on enums.
impl Message {
    fn call(&self) {
        println!("{:#?}", self)
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let office = IpAddrKind::V6("123.45.67.89".to_string());

    let hotel = Ipv4Addr {};

    // You can put any kind of data inside an enum variant: strings, numeric types, or structs,
    // for example. You can even include another enum!
    route(home);
    route(office);
    route(IpAddrKind::V4Struct(hotel));
    println!("Hello, world!");

    let a = Message::Write(String::from("Hello"));
    let b = Message::Quit;
    a.call();
    b.call();

    // The Option Enum
    // Option is another enum defined by the standard library.
    // The Option type encodes the very common scenario in which a value could be something, or it could be nothing.

    // Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly.
    // Its variants are also included in the prelude: You can use Some and None directly without the Option:: prefix.

    // The <T> syntax is a feature of Rust. It’s a generic type parameter.

    let some_number = Some(4); // Rust infers type Option<i32>
    let mut some_char = Some('a'); // Rust infers type Option<char>

    // let none = None; //Will throw a compile time error because The compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value.
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(4);
    // let sum = x+y; //  Rust doesn’t understand how to add an i8 and an Option<i8>, because they’re different types.

    // you have to convert an Option<T> to a T before you can perform T operations with it.
    // Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
    // Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.
    // This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.

    // To use an `Option<T>` value, your code must handle both variants.
    // You run specific logic only when the value is `Some(T)`.
    // In the `Some(T)` case, you are allowed to use the inner value of type `T`.
    // You also run separate logic when the value is `None`.
    // In the `None` case, no value of type `T` exists to work with.
    // `match` is a control flow construct built for enums like `Option`.
    // It executes different code depending on whether the variant is `Some` or `None`.
    // When a variant matches, you can access the data inside it if available.
}

/**
 * Takes any IpAddrKind (V4 or V6)
 * both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type
 */
fn route(ip: IpAddrKind) {
    println!("The value of ip address is {:#?}", ip);
}

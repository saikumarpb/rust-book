fn main() {
    // The Slice Type

    // Slices let you reference a contiguous sequence of elements in a collection.
    // A slice is a kind of reference, so it does not have ownership.

    // String Slices

    // A string slice is a reference to a contiguous sequence of the elements of a String.
    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}");

    // We create slices using a range within square brackets by specifying [starting_index..ending_index],
    // where starting_index is the first position in the slice and ending_index is one more than the last position in the slice.
    // Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index.

    // With Rust’s .. range syntax, if you want to start at index 0, you can drop the value before the two periods.
    let hello = &s[..5];
    // By the same token, if your slice includes the last byte of the String, you can drop the trailing number.
    let world = &s[6..];
    println!("{hello} {world}");

    // You can also drop both values to take a slice of the entire string. So, these are equal:
    let hello_world = &s[..];
    println!("{hello_world}");

    // Note: String slice range indices must occur at valid UTF-8 character boundaries.
    // If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

    let fw = first_word(&s);
    // s.clear(); this will throw a compilation error
    println!("{fw}");

    let s = "Hello, world!";
    println!("{s}");
    // The type of s here is &str: It’s a slice pointing to that specific point of the binary.
    // This is also why string literals are immutable; &str is an immutable reference.

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // Other Slices
    // String slices, as you might imagine, are specific to strings. But there’s a more general slice type too. Consider this array:
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[2..4];
    assert_eq!(a_slice, &[3, 4]);

    //  Let’s move on to Chapter 5 and look at grouping pieces of data together in a struct.
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
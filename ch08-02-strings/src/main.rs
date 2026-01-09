fn main() {
    // Storing UTF-8 Encoded Text with Strings

    // We discuss strings in the context of collections because strings are implemented as a collection of bytes,
    // plus some methods to provide useful functionality when those bytes are interpreted as text.

    //
    // Defining Strings

    // We’ll first define what we mean by the term string.
    // Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form, &str

    // The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

    // &str = just a reference (pointer + length), not the text.
    // &str itself is stored where your variable is stored (stack, struct field, etc.)
    // But the actual text it points to lives somewhere else, and that depends on the source of the string.

    //
    // Creating a New String

    // Many of the same operations available with Vec<T> are available with String as well because
    // String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.

    let mut s = String::new(); // An example of a function that works the same way with Vec<T> and String is the new function to create an instance.

    let data = "intial content";
    let s = data.to_string();

    let s = "initial content".to_string(); // works on a literal directly

    // to_string method, is available on any type that implements the Display trait.
    // We often use to_string method when we have some initial data with which we want to start the string

    // We can also use the function String::from to create a String from a string literal.
    let s = String::from("intitial content");

    // Strings are widely used, so Rust provides multiple generic string APIs.
    // Some may seem redundant, but each has its own place.
    // String::from and to_string do the same thing, so which one you choose is a matter of style and readability.

    // strings are UTF-8 encoded, so we can include any properly encoded data in them,
    let hello = String::from("नमस्ते"); // valid string value

    println!("Hello in hindi: {hello}");

    //
    // Updating a String

    // A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you push more data into it.
    // In addition, you can conveniently use the + operator or the format! macro to concatenate String values.
    let s = s + ", updated";

    println!("{0:?}", &s);

    // Appending with push_str or push

    let mut s1 = String::from("Hello");

    let s2 = String::from(" World");

    s1.push_str(s2.as_str()); // push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter.

    println!("S1 is {s1:?}");
    println!("S2 is {s2:?}"); // If the push_str method took ownership of s2, we wouldn’t be able to print its value

    // The push method takes a single character as a parameter and adds it to the String
    s1.push('!');
    println!("S1 is {s1:?}");

    //
    // Concatenating with + or format!
    let s1 = String::from("Hello ");
    let s2 = String::from("World!");

    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator.
    // The + operator uses the add method, whose signature looks something like this:
    //
    // fn add(self, s: &str) -> String {

    // `s2` has a leading `&`, meaning we pass a **reference** of the second string.
    // `String` can only be added with a `&str` (string slice), not another `String`.
    // The type of `&s2` is `&String`, not `&str`, so direct types don’t match `add(&str)`.
    // This compiles because Rust applies **Deref coercion** automatically.
    // The compiler converts `&String` → `&str` by dereferencing into a slice: `&s2[..]`.
    // `add()` does **not take ownership**, so `s2` remains valid after the operation.
    // Rust uses deref coercion here, a feature explained further in Chapter 15.

    // If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic"); // redifining s1 because s1 is moved into s4 above

    // For combining strings in more complicated ways, we can instead use the format! macro:
    let s5 = format!("{s1}-{s2}-{s3}");
    // format! macro uses references so that this call doesn’t take ownership of any of its parameters.

    println!("s5 -> {s5}, s1 -> {s1}, s2 -> {s2}, s3 -> {s3}");

    //
    // Indexing into Strings

    //  Rust strings don’t support indexing. But why not? To answer that question, we need to discuss how Rust stores strings in memory.

    // Internal Representation of a String : A String is a wrapper over a Vec<u8>

    let hello = String::from("Hola"); //  properly encoded UTF-8
    // In this case, len will be 4, which means the vector storing the string "Hola" is 4 bytes long. Each of these letters takes 1 byte when encoded in UTF-8.

    let hello = String::from("Здравствуйте"); // note that this string begins with the capital Cyrillic letter Ze, not the number 3
    // In this case, len will be 24, That’s the number of bytes it takes to encode “Здравствуйте” in UTF-8
    //  because each Unicode scalar value in that string (Здравствуйте) takes 2 bytes of storage.

    // Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.

    // let answer = &hello[0]; gives compile error

    // UTF-8 strings can be viewed in Rust in 3 relevant ways:

    // 1) raw bytes (`u8`), 2) Unicode scalar values (`char`), 3) grapheme clusters (user-perceived letters).
    // Example with Hindi word “नमस्ते”:
    // Bytes: 18 UTF-8 encoded bytes stored contiguously on the heap.
    // Bytes: 18 → [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    // 2) Scalars (`char`): 6 Unicode scalar values ['न', 'म', 'स', '्', 'त', 'े'], including diacritics like '्' that aren’t letters by themselves.

    // Grapheme clusters: 4 user-perceived letters: ["न", "म", "स्", "ते"].
    // Rust offers these interpretations so programs can choose what fits their logic,
    // regardless of the human language the string represents.

    // A final reason Rust doesn’t allow us to index into a `String` to get a character is that
    // indexing is expected to always take constant time (O(1)).
    // But it isn’t possible to guarantee that performance with a `String`,
    // because Rust would have to walk through the contents from the beginning to the index
    // to determine how many valid characters there were.

    //
    // Indexing a `String` is often a bad idea because the return type is ambiguous:
    // a byte, a character, a grapheme cluster, or a slice.
    // If indices are needed for slicing, Rust requires explicit ranges.
    // Instead of `s[i]`, use `s[start..end]` to slice exact bytes.
    // Ranged indexing returns a `&str` slice representing those bytes.

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{s}"); // Will print only 2 letters "Зд" because each of this letter takes 2 bytes

    // let s = &hello[0..3]; // Panics: byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравствуйте`
    // Rust would panic at runtime in the same way as if an invalid index were accessed in a vector:

    //
    //
    // Iterating Over Strings

    // The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.

    // For individual Unicode scalar values, use the chars method.

    let hello = "Здравствуйте";
    for c in hello.chars() {
        println!("{c}")
    }

    // Alternatively, the bytes method returns each raw byte

    for b in "Зд".bytes() {
        println!("{b}")
    }

    // prints
    // 208
    // 151
    // 208
    // 180

    // But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.

    // Getting grapheme clusters from strings (like in Devanagari) is complex,
    // so the standard library does not provide this functionality.
    // You can use external crates from crates.io (crates.io / crates.io search) if you need it.

    // useful methods like contains for searching in a string and replace for substituting parts of a string with another string.
}

use ch08_03_hash_maps::{employee_directory, pig_latin, statistics};
use std::collections::HashMap;

fn main() {
    // The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function,
    // which determines how it places these keys and values into memory.

    // Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.

    //
    // Creating a New Hash Map

    // Note that we need to first use the HashMap from the collections portion of the standard library.
    // Of our three common collections, this one is the least often used, so it’s not included in the features brought into scope automatically in the prelude.
    // Hash maps also have less support from the standard library; there’s no built-in macro to construct them.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 100);
    scores.insert(String::from("Yellow"), 20);

    // Just like vectors, hash maps store their data on the heap
    // Like vectors, hash maps are homogeneous: All of the keys must have the same type, and all of the values must have the same type.

    //
    // Accessing Values in a Hash Map

    let team_name = String::from("Blue");

    let score: Option<&i32> = scores.get(&team_name); // get method returns an Option<&V>, if there’s no value for that key in the hash map, get will return None.

    let score: Option<i32> = scores.get(&team_name).copied(); // calling copied will get an Option<i32> rather than an Option<&i32>

    // let score = scores.get("kfgvks").copied().unwrap(); // panicks : called `Option::unwrap()` on a `None` value

    let score = scores.get(&team_name).copied().unwrap_or(0); // unwrap_or to set score to zero if scores doesn’t have an entry for the key.

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //
    // Managing Ownership in Hash Maps

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of those values

    let field_name = String::from("K1");
    let field_value = String::from("V1");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{field_name}: {field_value}"); // field_name and field_value are invalid at this point, they’ve been moved into the hash map with the call to insert.

    // If we insert references to values into the hash map, the values won’t be moved into the hash map.
    // The values that the references point to must be valid for at least as long as the hash map is valid.
    // We’ll talk more about these issues in “Validating References with Lifetimes” in Chapter 10.

    // observe the below code

    let mut field_name = String::from("K1");
    let field_value = String::from("V1");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value); // field_name` is borrowed here

    // field_name = String::from("sfsf"); // `field_name` is assigned to here but it was already borrowed

    for (key, value) in &map
    // borrow later used here
    {
        println!("{key}: {value}");
    }

    //
    // Updating a Hash Map

    // When modifying a hash map, you must decide what to do if a key already exists.
    // One option is to replace the old value with the new one.
    // Another option is to keep the old value and ignore the new one if the key exists.
    // A third option is to combine the old value with the new value.
    // Rust provides APIs to handle each of these cases.

    // Overwriting a Value
    // If we insert a key and a value into a hash map and then insert that same key with a different value,
    // the value associated with that key will be replaced.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // The original value of 10 has been overwritten.

    println!("{scores:?}");

    // Adding a Key and Value Only If a Key Isn’t Present

    // A common pattern is to check whether a key already exists in a hash map.
    // If the key exists, keep the existing value unchanged.
    // If the key does not exist, insert the key with its value.

    // Hash maps have a special API for this called entry that takes the key you want to check as a parameter.
    // The return value of the entry method is an enum called Entry that represents a value that might or might not exist.

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // `Entry::or_insert` returns a mutable reference to the value for the key.
    // If the key exists, it returns a mutable reference to the existing value.
    // If the key does not exist, it inserts the given value and returns a mutable reference to it.
    // This approach is cleaner than manual checks and works smoothly with the borrow checker.

    scores.entry(String::from("Yellow")).or_insert(30);
    let x = scores.entry(String::from("Blue")).or_insert(50); // will not change the hash map, because the Blue team already has the value 10.

    println!("{x}");
    println!("{scores:?}");

    // Updating a Value Based on the Old Value
    // Another common use case for hash maps is to look up a key’s value and then update it based on the old value.

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // The or_insert method returns a mutable reference (&mut V) to the value for the specified key. 
        *count += 1; // so in order to assign to that value, we must first dereference count using the asterisk (*). 
    }

    println!("{map:?}");

    // By default, `HashMap` uses SipHash, which is resistant to hash table DoS attacks.
    // SipHash is not the fastest hashing algorithm, but it prioritizes security over speed.
    // This security–performance trade-off is usually worth it for general use.
    // If profiling shows hashing is a bottleneck, you can switch to a different hash function.
    // You do this by specifying a custom hasher.
    // A hasher is any type that implements the `BuildHasher` trait.
    // You don’t need to implement one from scratch — crates.io provides many ready-made hashers.
    // Traits and their implementations are covered in Chapter 10.

    //--------------------------------------------------------------------------------------------
    excercise1();
    excercise2();
    excercise3();
}

fn excercise1() {
    // Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

    let mut integers = vec![3, 4, 8, 2, 2, 5, 0, 9, 6, 6, 1, 10, 2];
    let median = statistics::median(&mut integers);
    let (mode, count) = statistics::mode(&mut integers);

    println!("{integers:?}, median = {median}, mode = {mode} with count : {count}");
}

fn excercise2() {
    // Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word and ay is added, \
    // so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
    let str = "first apple cbnm because".to_string();

    let pig_latin_str = pig_latin::convert_to_pig_latin(&str);

    println!("{pig_latin_str}");
}

fn excercise3() {
    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
    // for example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then, let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

    employee_directory::employee_directory_program();
}

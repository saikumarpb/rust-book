fn main() {
    // A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable.
    // Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
    let s1 = String::from("Maverick");
    let length = calculate_length(&s1);

    println!("s1: {s1}, length of s1: {length}");

    // The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *

    // References don’t drop data when they go out of scope because they don’t own it.

    // We call the action of creating a reference borrowing.

    // Just like variables, references are immutable by default.

    // Mutable References
    let mut s2 = String::from("Hello");
    change(&mut s2);

    println!("s2: {s2}");

    // Mutable references have one big restriction: If you have a mutable reference to a value, you can have no other references to that value.

    // The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

    // Two or more pointers access the same data at the same time.
    // At least one of the pointers is being used to write to the data.
    // There’s no mechanism being used to synchronize access to the data.

    // a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

    let mut s3 = String::from("Hello, ");
    let r1 = &s3;
    let r2 = &s3;
    println!("r1: {r1}, r2: {r2}");

    // r1, r2 are aut of scope now as its not being used after this point in thuis program
    let r3 = &mut s3;
    println!("r3: {r3}");

    // Dangling References

    // let ref_to_nothing = dangle();

    let ref_to_string = no_dangle();
    println!("ref_to_string: {ref_to_string}");

    // The Rules of References
    //  At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", World");
}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }
// Throws an error because, the owner of the reference being returned from dangle goes out of scope when the function ends

fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}

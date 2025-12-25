fn main() {
    // What Is Ownership?
    // Ownership is a set of rules that govern how a Rust program manages memory.

    //Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs;
    // in other languages, the programmer must explicitly allocate and free the memory.
    // Rust uses a third approach: Memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.

    // The Stack and the Heap

    // All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
    // The heap is less organized: When you put data on the heap, you request a certain amount of space.
    // The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
    // This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating).
    // Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer

    // Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
    // Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

    // Accessing data in the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there
    // a processor can usually do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap).

    // Keeping track of what parts of code are using what data on the heap,
    // minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so that you don’t run out of space are all problems that ownership addresses.

    // Ownership Rules
    // 1. Each value in Rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    let mut x = "dskjfhgsd";
    x = "sfghsdkgbhsdklhgbklsd";

    println!("{x}");

    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack.
    // We do not copy the data on the heap that the pointer refers to.
    let s1 = String::from("Sai");
    let s2 = s1;
    // println!("The value of s1 is : {s1}")

    // If you’ve heard the terms shallow copy and deep copy while working with other languages,
    // the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy.
    // But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.

    // Scope and Assignment

    // When you assign a completely new value to an existing variable,
    // Rust will call drop and free the original value’s memory immediately.
    let mut s = String::from("Hello");
    s = String::from("Sai");

    // The original string ("Hello") thus immediately goes out of scope.
    // Rust will run the drop function on it and its memory will be freed right away.
    println!("{s}, World!");

    // Variables and Data Interacting with Clone

    // If we do want to deeply copy the heap data of the String, not just the stack data,
    // we can use a common method called clone.
    let u1 = String::from("U1 Data");
    let u2 = u1.clone();
    println!("u1: {u1}, u2:{u2}");
    // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.

    // Stack-Only Data: Copy
    let l = 5;
    let m = l;
    println!("l: {l}, m: {m}");

    // But this code seems to contradict what we just learned: We don’t have a call to clone, but l is still valid and wasn’t moved into m.

    // The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
    // That means there’s no reason we would want to prevent x from being valid after we create the variable y.
    // In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.

    // as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy.
    // Here are some of the types that implement Copy:

    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    // Ownership and Functions

    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
    // Passing a variable to a function will move or copy, just as assignment does.

    let a = String::from("A");
    takes_ownership(a);
    // a is no longer valid here, as its moved!

    // println!({a}); // will be a compile time error

    let b = 6;
    makes_copy(b);
    // b is still valid here as it isn't moved, just copied!

    // Return Values and Scope
    // Returning values can also transfer ownership.
    let hello = gives_ownership();
    println!("hello: {hello}");

    // taking ownership and then returning ownership with every function is a bit tedious.
    // Rust has a feature for using a value without transferring ownership: references.
}

fn takes_ownership(some_string: String) {
    println!("some_string: {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {some_integer}");
}

fn gives_ownership() -> String {
    let x = String::from("Hello");
    x
}

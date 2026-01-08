enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Common Collections

    // Most Rust data types hold one value, but collections hold many.
    // Unlike arrays & tuples, collections allocate on the **heap**.
    // Heap storage removes the need to know size at compile time.
    // This allows collections to **grow or shrink at runtime**.

    // We will discuss 3 collections that are used very often in Rust programs:

    // A vector allows you to store a variable number of values next to each other.
    // A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter, we’ll talk about it in depth.
    // A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.

    //
    // Vectors
    // Vec<T>, also known as a vector. Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
    // Vectors can only store values of the same type.

    let v1: Vec<i32> = Vec::new(); // Observe that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.

    // Vectors are implemented using generics; Vec<T> can hold any type

    // To create a vector with initial values we can use vec! macro
    let mut v2 = vec![1, 2, 3]; // here type i32 is inferred by rust because we provided initial values

    v2.push(4); // to add elements to vector , we can use push method
    // observe that v2 is made mutable because variables are immutable by default, to be able to add/change the value to vector we need to make it mutable (mut)

    // a pop method removes and returns the last element.

    //
    // Reading Elements of Vectors
    // There are two ways to reference a value stored in a vector: via indexing or by using the get method.

    let third = &v2[2];
    println!("The third value of v2 is {third}");

    let third: Option<&i32> = v2.get(2); // added type annotation for extra clarity
    // observe that the type is Option<&T> when using the get method to read the value from vector

    match third {
        Some(val) => println!("The third value of v2 is {val}"),
        None => println!("There's no third element"),
    }

    // let does_not_exist = &v2[100]; // This will cause the program to panic because it references a nonexistent element. This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.

    // v2.push(5); // Compile error : cannot borrow `2` as mutable because it is also borrowed as immutable

    println!("The third value of v2 is {third:?}");

    // The borrow checker ensures vector references stay valid by enforcing ownership/borrowing rules.
    // You can’t hold an immutable reference to a vector element and then mutate the vector in the same scope.
    // Example: referencing `vec[0]` immutably and later pushing (`vec.push`) fails because mutation can invalidate the borrow.
    // Rust prevents this at compile time if the borrowed element is used again after the mutation.

    //
    // Why should a reference to the first element care about changes at the end of the vector?
    // Because vectors store elements next to each other in contiguous heap memory,
    // adding a new element to the end (`vec.push()`) might require allocating new memory
    // and copying the old elements into the new space if there isn't enough room
    // to keep all elements next to each other in the current allocation.
    // In that case, the reference to the first element would point to deallocated memory.
    // The borrowing rules prevent programs from ending up in that situation.

    //
    // Iterating Over the Values in a Vector
    for i in &v2
    // for loop to get immutable references to each element in a vector of i32 values and print them.
    {
        println!("{i}")
    }

    // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
    for i in &mut v2 {
        *i += 50; // To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator.
        println!("{i}");
        // v2.push(3); // If we attempted to insert or remove items in the for loop bodies, we would get a compiler error
        // The reference to the vector that the for loop holds prevents simultaneous modification of the whole vector
    }

    // Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker’s rules.

    //
    // Using an Enum to Store Multiple Types

    // Vectors can only store values that are of the same type, which can be inconvenient when you need mixed types.
    // Fortunately, enum variants belong to the same enum type,
    // so we can define and use an enum when we need one type to represent elements of different inner types.

    let row = vec![
        SpreadSheetCell::Int(1),
        SpreadSheetCell::Float(3.456),
        SpreadSheetCell::Text(String::from("Maverick")),
    ];

    // Rust must know the vector’s element type at compile time to determine the exact heap memory needed per element.
    // The vector must explicitly declare one allowed type for all elements.
    // If vectors could hold any type, operations on elements could fail due to unsupported behavior.
    // Enums solve this by providing one compile-time known type with multiple inner variants.
    // Using `match` with an enum ensures all possible variants are handled at compile time (exhaustive checking).

    // If you don’t know the exhaustive set of types at runtime, you can’t use an enum in a vector.
    // The enum technique won’t work in that case.
    // Instead, you can use a trait object (covered later in Chapter 18).

    // Dropping a Vector Drops Its Elements
    // Like any other struct, a vector is freed when it goes out of scope
}

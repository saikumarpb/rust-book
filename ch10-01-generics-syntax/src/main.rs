fn main() {
    // Every programming language provides ways to reduce duplication.
    //
    // In Rust, one such tool is generics, which act as abstract
    // placeholders for concrete types or other properties.
    //
    // Generics allow us to define behavior and relationships
    // without knowing the exact types that will be used
    // when the code is compiled or run.

    // We already used generics in Chapter 6 with Option<T>.
    //
    // In Chapter 8, we used generics with Vec<T> and HashMap<K, V>.
    //
    // In Chapter 9, we worked with Result<T, E>.
    //
    // In this chapter, we’ll explore how to define our own
    // types, functions, and methods using generics.

    // Finally, we’ll discuss lifetimes.
    //
    // Lifetimes are a kind of generics that give the compiler
    // information about how references relate to each other.
    //
    // They allow us to tell the compiler how long borrowed values live,
    // so it can ensure that references remain valid.
    //
    // With lifetimes, Rust can verify reference safety
    // in more situations than it could without our help.

    let num_list = [1, 12, 3, 4];

    let largest_num = largest_number(&num_list);
    println!("The largest num : {largest_num}");

    let char_list = ['a', 'z', 'b', 'd'];

    let largest_char = largest_char(&char_list);
    println!("The largest char : {largest_char}");

    let largest_num = largest(&num_list);
    println!("The largest num : {largest_num}");

    let largest_char = largest(&char_list);
    println!("The largest char : {largest_char}");

    //
    // In Struct Definitions

    // We can also define structs to use a generic type parameter in one or more fields using the <> syntax.
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 1, y: 2 };

    let float = Point { x: 1.2, y: 2.3 };

    // If we create an instance of a Point<T> that has values of different types

    // let wont_work =- Point {
    //     x: 1.0,
    //     y: 2
    // };

    // To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters.

    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point2 { x: 1, y: 2 };

    let both_float = Point2 { x: 2.1, y: 6.0 };

    let integer_and_float = Point2 { x: 1, y: 3.5 };

    // You can use as many generic type parameters in a definition as you want.
    //
    // However, using more than a few can make your code hard to read.
    //
    // If you find yourself needing many generic types,
    // it may be a sign that your code should be restructured
    // into smaller, more focused pieces.

    //
    //
    // In enum definitions, we can use generics just like we do with structs.
    //
    // Enums can hold generic data types inside their variants.
    // A common example is the Option<T> enum from the standard library:
    //
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    //
    // Option<T> is generic over the type T.
    // It has two variants:
    // - Some(T), which stores a value of type T
    // - None, which stores no value
    //
    // This allows Option<T> to represent the abstract idea of an optional value.
    // Because it’s generic, the same enum works for any type.
    //
    // Enums can also use multiple generic type parameters.
    // The Result enum is a good example:
    //
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    //
    // Result<T, E> is generic over two types:
    // - T for the success value
    // - E for the error value
    //
    // Ok(T) holds a successful result,
    // and Err(E) holds an error value.
    //
    // This makes Result useful for operations that can either succeed
    // (returning a value of type T) or fail
    // (returning an error of type E).
    //
    // For example, when opening a file:
    // - T becomes std::fs::File if the operation succeeds
    // - E becomes std::io::Error if the operation fails
    //
    // When you notice multiple struct or enum definitions
    // that differ only by the types they store,
    // you can avoid code duplication by using generics instead.

    //
    // In Method Definitions

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    println!("p.x = {}", integer.x());

    // This below code means the type Point<f32> will have a distance_from_origin method;
    // other instances of Point<T> where T is not of type f32 will not have this method defined.
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    println!("distance from origin = {}", float.distance_from_origin());

    struct Point3<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point3<X1, Y1> {
        fn mixup<X2, Y2>(self, other_point: Point3<X2, Y2>) -> Point3<X1, Y2> {
            Point3 {
                x: self.x,
                y: other_point.y,
            }
        }
    }

    let point1 = Point3 { x: 1, y: 2.0 };
    let point2 = Point3 { x: "Mav", y: 'x' };

    let p3 = point1.mixup(point2);

    println!("p3.x: {}, p3.y: {}", p3.x, p3.y);

    // The purpose of this example is to show a case where
    // some generic parameters are declared with `impl`
    // and others are declared with the method definition.
    //
    // The generic parameters X1 and Y1 are declared after `impl`
    // because they belong to the struct itself.
    //
    // The generic parameters X2 and Y2 are declared after `fn mixup`
    // because they are only relevant to that specific method.

    // Performance of Code Using Generics
    //
    // A common concern is whether generic type parameters
    // introduce runtime overhead.
    //
    // Using generic types does not make programs run slower
    // than code written with concrete types.
    //
    // Rust achieves this through monomorphization,
    // which occurs at compile time.
    //
    // Monomorphization is the process of converting
    // generic code into concrete, type-specific implementations.
    //
    // During compilation, the compiler identifies every place
    // where generic code is used and generates specialized code
    // for each concrete type.
    //
    // As a result, the final binary contains fully concrete code
    // with no runtime cost for generics.

    // Example: Monomorphization with Option<T>
    //
    // Consider the following generic usage:
    //
    // let integer = Some(5);
    // let float = Some(5.0);
    //
    // During compilation, Rust performs monomorphization.
    //
    // In this process, the compiler examines how Option<T>
    // is used and identifies the concrete types involved.
    //
    // In this case, Option<T> is instantiated with:
    //  - i32
    //  - f64
    //
    // The compiler then expands the generic Option<T>
    // into separate, type-specific definitions.
    //
    // The generated code is conceptually similar to:
    //
    // enum Option_i32 {
    //     Some(i32),
    //     None,
    // }
    //
    // enum Option_f64 {
    //     Some(f64),
    //     None,
    // }
    //
    // fn main() {
    //     let integer = Option_i32::Some(5);
    //     let float = Option_f64::Some(5.0);
    // }
    //
    // The actual compiler-generated names differ,
    // but the behavior is equivalent.
    //
    // The generic Option<T> is replaced entirely
    // with concrete implementations.
    //
    // Because the final compiled code contains
    // only concrete types, there is no runtime cost
    // for using generics.
    //
    // Execution performance is the same as if
    // each type-specific definition had been
    // written manually.
    //
    // This monomorphization process is what makes
    // Rust’s generics extremely efficient at runtime.
}

fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

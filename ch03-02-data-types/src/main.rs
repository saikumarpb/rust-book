fn main() {
    // Rust is a statically typed language, which means that it must know the types of all variables at compile time

    // --------------Scalar Types------------------
    // A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

    // Integer Types
    // Length	                Signed	Unsigned
    // 8-bit	                i8	    u8
    // 16-bit	                i16	    u16
    // 32-bit	                i32	    u32
    // 64-bit	                i64	    u64
    // 128-bit	                i128	u128
    // Architecture-dependent	isize	usize

    // Each signed variant can store numbers from -(2^(n - 1)) to 2^(n - 1) inclusive, where n is the number of bits that variant uses.
    // Unsigned variants can store numbers from 0 to 2^(n − 1)
    // isize and usize types depend on the architecture of the computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

    // Floating-Point Types
    // Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64
    // All floating-point types are signed.

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    let remainder = 43 % 5;

    println!(
        "sum: {sum}, difference:{difference}, product:{product}, quotient:{quotient}, truncated:{truncated}, remainder:{remainder}"
    );

    // Boolean Type
    //  Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool.

    let t = true;
    let f: bool = false;

    println!("t:{t}, f:{f}");

    // The Character Type
    // we specify char literals with single quotation marks, as opposed to string literals, which use double quotation marks.
    // char type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII.
    // Accented letters; Chinese, Japanese, and Korean characters; emojis; and zero-width spaces are all valid char values in Rust.

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c:{c}, z:{z}, heart_eyed_cat:{heart_eyed_cat}");

    // --------------Compound Types------------------
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // The Tuple Type
    // Tuples have a fixed length: Once declared, they cannot grow or shrink in size.

    let tup: (i32, f64, u8) = (-3, 334.2542, 3);
    let (u, v, w) = tup; // destructuring

    let minus_three = tup.0; // accessing using index
    let three = tup.2;
    println!("u:{u}, v:{v}, w:{w}, minus_three={minus_three}, three={three}");

    // The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type.
    // Expressions implicitly return the unit value if they don’t return any other value.

    // The Array Type
    // Unlike a tuple, every element of an array must have the same type. arrays in Rust have a fixed length.
    // For arrays data is allocated on stack.
    // A vector is a similar data type provided by standard library that is allowed to shrink or grow in size because its contents live on stack.

    let _a = [1, 2, 3, 4, 5];
    let _b: [i32; 3] = [1, 2, 3]; // write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array
    let c = [3; 5]; // This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
    let d = c[4];
    println!("{d}")
}

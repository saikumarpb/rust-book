use std::fmt::{Debug, Display};

fn main() {
    // A trait defines the functionality a particular type has and can share with other types.
    // We can use traits to define shared behavior in an abstract way.
    // We can use trait bounds to specify that a generic type can be any type that has certain behavior.

    // Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

    //
    // Defining a Trait

    // A type’s behavior consists of the methods that can be called on that type.
    //
    // Different types share the same behavior when the same methods
    // can be called on all of them.
    //
    // Trait definitions provide a way to group method signatures together
    // to define a common set of behaviors.
    //
    // These behaviors represent what a type must implement
    // in order to fulfill a particular purpose.

    pub trait Summary {
        fn summarize(&self) -> String;

        fn summarize_author(&self) -> String;

        // Using Default Implementations
        //
        // Sometimes it’s useful to provide default behavior for some or all methods
        // in a trait, instead of requiring every type to implement every method.
        //
        // When implementing the trait for a specific type,
        // each method can either:
        //
        // - use the default implementation, or
        // - override the default behavior with a custom implementation.

        // A default implementation in a trait does not require any changes to existing implementations.
        // This is because overriding a default method uses the same syntax as implementing a method without a default.
        // If a type provides its own implementation, that version is used; otherwise, the trait’s default implementation is applied automatically.
        fn summarize_default(&self) -> String {
            format!("Read more from {} ...", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }

    // Implementing a Trait on a Type

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{} by {} ({})", self.headline, self.author, self.location)
        }

        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }

        fn summarize_default(&self) -> String {
            format!("{} by {} ({})", self.headline, self.author, self.location)
        }
    }

    impl Summary for SocialPost {
        fn summarize(&self) -> String {
            format!("{} : {}", self.username, self.content)
        }

        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let post = SocialPost {
        username: String::from("Maverick"),
        content: String::from("Hello, Ladies and Gentlemen, Today we are training for dog fight"),
        reply: false,
        repost: false,
    };

    println!("Summary of post :\n{}", post.summarize());
    println!("Default summary: {}", post.summarize_default());

    // Other crates that depend on the `aggregator` crate can bring the `Summary`
    // trait into scope and implement `Summary` for their own types.
    //
    // There is one important restriction:
    //
    // A trait can be implemented for a type only if
    // either the trait or the type (or both) are local to the current crate.
    //
    // Examples that are allowed:
    //
    // - Implementing a standard library trait like `Display` on a custom type
    //   such as `SocialPost`, because `SocialPost` is local to the aggregator crate.
    //
    // - Implementing the `Summary` trait on `Vec<T>` inside the aggregator crate,
    //   because the trait `Summary` itself is local to the crate.
    //
    // What is NOT allowed:
    //
    // - Implementing an external trait on an external type.
    //   For example, implementing `Display` for `Vec<T>` is not allowed,
    //   because both `Display` and `Vec<T>` are defined in the standard library.
    //
    // This restriction is part of Rust’s coherence rules,
    // more specifically known as the orphan rule.
    //
    // The orphan rule exists to prevent conflicts between crates.
    // Without it, two different crates could implement the same trait
    // for the same type, and Rust would not know which implementation to use.

    //
    // Using Traits as Parameters

    // The notify function accepts a parameter named `item`.
    // Instead of a concrete type, it uses `impl Summary`.

    // This means `item` can be any type
    // as long as it implements the `Summary` trait.
    pub fn notify(item: &impl Summary) {
        // Because `item` implements Summary,
        // we are guaranteed that the `summarize` method exists.
        println!("Breaking news: {}", item.summarize());
    }

    notify(&post);

    // Trait Bound Syntax
    // The impl Trait syntax above works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound; it looks like this:

    fn notify_trait_bound_syntax<T: Summary>(item: &T) {
        println!("Breaking news: {}", item.summarize());
    }

    // `impl Trait` syntax is shorter and easier to read in simple cases.

    // Here, both parameters must implement the `Summary` trait,
    // but they are allowed to be different concrete types.
    pub fn notify_v2(item1: &impl Summary, item2: &impl Summary) {
        // item1 and item2 can be different types
        // as long as both implement Summary
    }

    // Trait bound syntax is more explicit and more powerful.

    // Here, the generic type `T` must implement `Summary`.
    pub fn notify_v3<T: Summary>(item1: &T, item2: &T) {
        // The generic type `T` is used for both parameters.
        //
        // This constrains the function so that:
        // - item1 and item2 must be the SAME concrete type
        // - that type must implement the `Summary` trait
        //
        // Example:
        // ✅ notify(&tweet, &tweet)
        // ❌ notify(&tweet, &news_article)  // different types not allowed
    }

    // Multiple Trait Bounds with the + Syntax

    pub fn notify_v4(item: &(impl Summary + Display)) {
        // Here, `item` must implement BOTH traits:
        // - Summary
        // - Display
        //
        // This allows the function to:
        println!("{}", item); // use `{}` formatting from Display
        println!("{}", item.summarize()); // call `summarize()` from Summary
    }

    //
    // The + syntax is also valid with trait bounds on generic types:
    pub fn notify_v5<T: Summary + Display>(item: &T) {
        println!("{}", item); // use `{}` formatting from Display
        println!("{}", item.summarize()); // call `summarize()` from Summary
    }

    //
    // Clearer Trait Bounds with where Clauses

    // Using too many trait bounds can reduce readability. When multiple generic parameters each have their own trait bounds,
    // the function signature can become long and difficult to read.
    // To address this, Rust provides an alternative syntax using a where clause, which allows trait bounds to be written after the function signature.

    // So, instead of writing this:
    fn some_function_v1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

    // we can use a where clause, like this:
    fn some_function_v2<T, U>(t: T, u: U)
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        // This function signature is less cluttered because the function name, parameter list, and return type remain close together,
        // making it easier to read—similar to a function without many trait bounds.
    }

    //
    // Returning Types That Implement Traits

    // Returning a type using only the trait it implements is especially useful
    // with closures and iterators which will be covered in Ch 13.
    //
    // Closures and iterators often create types that:
    // - are known only to the compiler, or
    // - are extremely long and complex to write explicitly.
    //
    // The `impl Trait` syntax allows a function to say:
    // “this returns some type that implements `Iterator`”
    //
    // This avoids writing long, unreadable return types
    // while still keeping full type safety.

    fn returns_summarizable() -> impl Summary {
        SocialPost {
            username: String::from("Maverick"),
            content: String::from("of course"),
            reply: false,
            repost: false,
        }
    }

    // `impl Trait` can be used only when a function returns a single concrete type.
    //
    // Even though both `NewsArticle` and `SocialPost` implement the `Summary` trait,
    // the compiler still requires the function to return exactly one concrete type.
    //
    // The following code does NOT compile because:
    // - one branch returns `NewsArticle`
    // - the other branch returns `SocialPost`
    //
    // fn returns_summarizable(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle {
    //             headline: String::from(
    //                 "Penguins win the Stanley Cup Championship!",
    //             ),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //                  hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         SocialPost {
    //             username: String::from("horse_ebooks"),
    //             content: String::from(
    //                 "of course, as you probably already know, people",
    //             ),
    //             reply: false,
    //             repost: false,
    //         }
    //     }
    // }
    //
    //
    // `impl Summary` means:
    // “some one specific type that implements Summary”
    // not
    // “any type that implements Summary”
    //
    // Because the compiler must choose one concrete return type,
    // returning different types from different branches is not allowed.
    //
    // This limitation exists due to how `impl Trait` is implemented internally.
    //
    // To return different types that share the same behavior,
    // Rust uses trait objects (`dyn Trait`),
    // which are covered in Chapter 18.

    //
    // ===============================
    // Using Trait Bounds for Methods
    // ===============================

    // A generic struct that can store any type `T`
    struct Pair<T> {
        x: T,
        y: T,
    }

    // --------------------------------
    // impl<T> → applies to ALL types T
    // --------------------------------
    //
    // No trait bounds are required here.
    // This means these methods are always available,
    // regardless of what type T is.

    impl<T> Pair<T> {
        // `Self` refers to `Pair<T>`
        // This method works for every T because
        // it only stores values and performs no operations on them.
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // ---------------------------------------------------------
    // impl<T: Display + PartialOrd>
    // ---------------------------------------------------------
    //
    // This implementation is CONDITIONAL.
    //
    // The method inside this impl block exists ONLY if:
    //
    //   T implements:
    //   - Display     → allows printing with `{}`
    //   - PartialOrd  → allows comparison like >=, <=
    //
    // If T does not implement these traits,
    // this method will not be available.

    use std::fmt::Display;

    impl<T: Display + PartialOrd> Pair<T> {
        // This method is available only for types
        // that can be compared and printed.
        fn cmp_display(&self) {
            // Requires `PartialOrd`
            if self.x >= self.y {
                // Requires `Display`
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // --------------------------------
    // Summary
    // --------------------------------
    //
    // • Pair<T> always has:
    //     - new()
    //
    // • Pair<T> has cmp_display() ONLY when:
    //     T: Display + PartialOrd
    //
    // • The compiler decides method availability at compile time.
    //
    // • No runtime checks are involved.
    //
    // • This is called:
    //     "conditional method implementation"
    //
    // • Trait bounds control WHAT methods exist,
    //   not just whether code runs

    //
    //
    // =====================================
    // Blanket Implementations
    // =====================================

    // A blanket implementation means:
    // implementing a trait for ALL types that satisfy certain trait bounds.

    // -------------------------------------
    // Example from the Rust standard library
    // -------------------------------------

    // impl<T: Display> ToString for T {
    // }

    // This means:
    //
    // Any type T that implements `Display`
    // automatically gets `ToString`.

    // -------------------------------------
    // Result of this blanket implementation
    // -------------------------------------

    // Since integers implement `Display`:
    let s = 3.to_string();

    // `i32` does NOT manually implement `ToString`.
    // The method exists because of the blanket impl:
    //
    //     impl<T: Display> ToString for T

    // -------------------------------------
    // Where to find blanket implementations
    // -------------------------------------

    // In documentation, blanket implementations appear under the trait’s “Implementors” section.

    // -------------------------------------
    // Why blanket implementations are useful
    // -------------------------------------

    // • Reduce code duplication
    // • Provide shared behavior automatically
    // • Extend functionality without modifying types
    // • Widely used throughout the standard library

    // -------------------------------------
    // Role of traits and trait bounds
    // -------------------------------------

    // Traits + generics allow writing flexible code
    // while still enforcing required behavior.

    // Trait bounds tell the compiler:
    // “this generic type must support these methods”.

    // -------------------------------------
    // Compile-time safety
    // -------------------------------------

    // Rust checks trait bounds at compile time.
    //
    // If a type does not implement a required trait,
    // the program will NOT compile.

    // This avoids runtime errors such as:
    // “method not found on type”.

    // -------------------------------------
    // Compared to dynamic languages
    // -------------------------------------

    // Dynamic languages:
    // • Errors appear at runtime
    // • Method existence is checked during execution

    // Rust:
    // • Errors appear at compile time
    // • Behavior is guaranteed before running the program

    // -------------------------------------
    // Performance benefit
    // -------------------------------------

    // No runtime method lookup is needed. No behavior checks during execution.
    //
    // Trait bounds + generics give:
    //
    // • safety
    // • performance
    // • flexibility
    //
    // all at the same time.
}

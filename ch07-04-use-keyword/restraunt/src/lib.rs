mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// We can create a shortcut to a path with the use keyword once and then use the shorter name everywhere else in the scope.
use front_of_house::hosting;

pub fn eat_at_restraunt() {
    hosting::add_to_waitlist()
}

mod customer {
    pub fn eat_at_restraunt() {
        // hosting::add_to_waitlist() // gives a compile error : use of undeclared crate or module `hosting`
        // This is beacause , customer module is a different scope than the use statement defined outside customer module.
    }
}

mod customer_mod_2 {
    // Now  use within the customer_mod_2 module too

    use super::front_of_house::hosting;

    pub fn eat_at_restraunt() {
        hosting::add_to_waitlist()
    }
}

// Creating Idiomatic use Paths

// we can also extend the use statement till the function name like below

use front_of_house::hosting::add_to_waitlist;

fn eat_at_restraunt_2() {
    add_to_waitlist()
}

// BUT BUT BUT
// The idiomatic way to bring a function into scope with `use` is to import its parent module, which keeps calls clear by showing the function isn’t locally defined while avoiding repetition of the full path.

// use front_of_house::hosting; is the IDIOMATIC WAY

// When importing structs, enums, and other non-function items with `use`, it’s idiomatic to specify the full path.

// For example

use std::collections::HashMap;

fn idiomatic_way_to_call_non_fn_items() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Rust doesn’t allow two `use` imports that bring items with the same name into the same scope, so you must disambiguate (e.g., import parents or alias one).

// For example
use std::fmt;
use std::io;

// Both fmt & io has Result type

fn function1() -> fmt::Result // we have used parent module to distinguish the two Result types.
{
    Ok(())
}

fn function2() -> io::Result<()> // we have used parent module to distinguish the two Result types.
{
    Ok(())
}

// If instead we specified use std::fmt::Result and use std::io::Result,
// we’d have two Result types in the same scope, and Rust wouldn’t know which one we meant when we used Result.

//
// Providing New Names with the as Keyword

// There’s another solution to the problem of bringing two types of the same name into the same scope with use:
// After the path, we can specify as and a new local name, or alias, for the type.

use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> fmt::Result {
    Ok(())
}

fn function4() -> IoResult<()> {
    Ok(())
}

//
// Re-exporting Names with pub use

// `use` imports are private to the scope by default.
// Code outside the module cannot access those imported names directly.
// `pub use` makes an import accessible outside the scope.
// This is called **re-exporting** (or re-export).
// It brings an item into scope and exposes it for other modules to import.
// Re-exporting lets others use the item as if it were defined in the current module.
// Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain.

pub use crate::front_of_house::hosting as hosting_re_exported;

// Before re-exporting, external code could only call `add_to_waitlist()` using the full path `restaurant::front_of_house::hosting::add_to_waitlist()`,
// which also required `front_of_house` to be `pub`. After adding `pub use` at the crate root to re-export `hosting`,
// the function becomes accessible via the shorter path `restaurant::hosting::add_to_waitlist()` without exposing the entire internal module tree.

// We’ll look at another example of pub use and how it affects your crate’s documentation in “Exporting a Convenient Public API” in Chapter 14.

//
// Using External Packages
use rand::Rng;

fn example_using_external_package() {
    let secret_number = rand::rng().random_range(1..=100);
}

// Observe that `std` is an external crate provided by Rust itself. Since it ships with the language, Cargo doesn’t require it to be added as a dependency in `Cargo.toml`.
// However, we still use the `use` keyword to bring items from `std` into our package’s scope before using them.


//
// Using Nested Paths to Clean Up use Lists

// Importing multiple items from the same crate or module on separate lines takes more vertical space.
// Rust provides **nested paths** to shorten these imports.
// We write the common path once, followed by `::`.
// Then wrap the differing item names inside `{}` as a comma-separated list.
// This brings multiple items into scope in a single `use` statement line.

use std::{cmp::Ordering, io as io2};

// use std::io;
// use std::io::Write;

// The common part of these two paths is std::io, and that’s the complete first path. 
// To merge these two paths into one use statement, we can use self in the nested path, as shown below

use std::io::{self as io3, Write};

//
// Importing Items with the Glob Operator

// If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:

use std::collections::*;

// Be careful using the glob operator (`*`) — it hides what’s in scope and where items are defined.
// If a dependency updates, glob imports may change too and break your code after upgrades.
// It can also cause name conflicts if the dependency adds an item matching your local names.
// Glob is commonly used in tests to import everything under test into the `tests` module.
// It’s also sometimes used in the **prelude** pattern from the standard library.

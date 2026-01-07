mod front_of_house;
// The compiler knows to look in src/front_of_house.rs file because it came across the module declaration in the crate root with the name front_of_house.

// The mod keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.

// `mod` is declared once to register a file in the module tree.
// It’s not an `include` copy-paste like other languages.
// Compiler remembers the file’s module location from where `mod` is placed.
// Other files access its items using the path from the declaration point.

pub use crate::front_of_house::hosting;

pub fn eat_at_restraunt() {
    hosting::add_to_wait_list()
}

// Rust supports two file path styles: the idiomatic `file.rs` and the older `mod.rs` style.
// For a module `front_of_house` declared at crate root, compiler looks in:
//  - `src/front_of_house.rs` (idiomatic)
//  - `src/front_of_house/mod.rs` (older, still supported)
// For a submodule `hosting` inside `front_of_house`, compiler looks in:
//  - `src/front_of_house/hosting.rs` (idiomatic)
//  - `src/front_of_house/hosting/mod.rs` (older, still supported)
// Using both styles for the *same* module causes a compiler error.
// Mixing styles across *different* modules is allowed, but can confuse project navigation.
// The main downside of `mod.rs` is many files end up named the same, making editors harder to navigate.

//
// ------ Summary/Recap ---------
// Rust lets you split a package into multiple crates, and crates into modules.
// Items in one module can be accessed from another using absolute (`crate::`) or relative (`super::`, `self::`, `module::`) paths.
// You can shorten paths in a scope by importing them once with `use`.
// Imports are private by default; `pub` makes structs, enums, fns, and `use` imports public (re-exporting).
// You expose items with `pub`, keeping modules encapsulated but accessible when needed.

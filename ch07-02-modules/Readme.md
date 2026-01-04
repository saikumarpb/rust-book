# Control Scope and Privacy with Modules

## Modules Cheat Sheet

**Start from the crate root:**
When compiling a crate, the compiler first looks in the crate root file (usually `src/lib.rs` for a library crate and `src/main.rs` for a binary crate) for code to compile.

**Declaring modules:**
In the crate root file, you can declare new modules; say you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:

* Inline, within curly brackets that replace the semicolon following `mod garden`
* In the file `src/garden.rs`
* In the file `src/garden/mod.rs`

**Declaring submodules:**
In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in `src/garden.rs`. The compiler will look for the submodule’s code within the directory named for the parent module in these places:

* Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
* In the file `src/garden/vegetables.rs`
* In the file `src/garden/vegetables/mod.rs`

**Paths to code in modules:**
Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code.
For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.

**Private vs. public:**
Code within a module is private from its parent modules by default.

* To make a module public, declare it with `pub mod` instead of `mod`.
* To make items within a public module public as well, use `pub` before their declarations.

**The use keyword:**
Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;`, and from then on you only need to write `Asparagus` to make use of that type in the scope.

### File Structure Example

```text
ch07-02-modules
└── backyard
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        ├── garden
        │   └── vegetables.rs
        ├── garden.rs
        └── main.rs           // crate root file
```

---

## Grouping Related Code in Modules

Modules let us organize code within a crate for readability and easy reuse. Modules also allow us to control the privacy of items because code within a module is private by default. Private items are internal implementation details not available for outside use. We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.

Inside modules, we can place other modules, as in this case with the modules `hosting` and `serving`. Modules can also hold definitions for other items, such as structs, enums, constants, traits, and functions. By using modules, we can group related definitions together and name why they’re related.

`src/main.rs` and `src/lib.rs` are called **crate roots**. The reason for their name is that the contents of either of these two files form a module named `crate` at the root of the crate’s module structure, known as the module tree.

**Example:** Create a new library named restaurant by running `cargo new restaurant --lib`

```text
crate
└── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    └── serving
        ├── take_order
        ├── serve_order
        └── take_payment

```

* `hosting` nests inside `front_of_house`
* `hosting` and `serving` are siblings defined within `front_of_house`

In Rust, a module inside another becomes its child (with the outer as parent). Observe that the full module tree is implicitly rooted under module name `crate`.

Rust’s module tree is very similar to a filesystem directory tree: modules organize code like directories, and like files inside directories, we need paths to locate and access them.

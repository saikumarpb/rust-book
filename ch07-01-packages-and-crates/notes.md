Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs.  
These features, sometimes collectively referred to as the module system, include:

**Packages**: A Cargo feature that lets you build, test, and share crates,  
**Crates**: A tree of modules that produces a library or executable,  
**Modules and use**: Let you control the organization, scope, and privacy of paths,  
**Paths**: A way of naming an item, such as a struct, function, or module,  

A crate is the smallest amount of code that the Rust compiler considers at a time.  
Even if you run `rustc` rather than cargo and pass a single source code file, the compiler considers that file to be a crate.  
Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.  

A crate can come in one of two forms:  
- **Binary crate**: Binary crates compile to runnable executables (CLI apps, servers, etc.) and must contain a `main()` function that defines the program’s entry point.  
- **Library crate**: Library crates don’t compile to executables or include `main()`; they provide reusable functionality to share across projects, like the `rand` crate.  

When Rustaceans say “crate,” they usually mean a library crate, using “crate” and “library” interchangeably.  

A package is a bundle of one or more crates that provides a set of functionality.  
A package contains a `Cargo.toml` file that describes how to build those crates.  
A package can contain as many binary crates as you like, but at most only one library crate.  
A package must contain at least one crate, whether that’s a library or binary crate.  

Cargo is actually a package that contains the binary crate for the command line tool `cargo` CLI (a binary crate).  
The Cargo package also contains a library crate that the binary crate depends on.  
Other projects can depend on the Cargo library crate to use the same logic the Cargo command line tool uses.  

Let’s walk through what happens when we create a package:  
`cargo new <PROJECT_NAME>`  
In the project directory, there’s a `Cargo.toml` file, giving us a package.  
There’s also a `src` directory that contains `main.rs`.  
Observe that in `Cargo.toml` there is **no mention of `src/main.rs`**.  

**Concept to know**:  
**Crate Root**: The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.  

Cargo treats:  
- `src/main.rs` as the root of the package’s binary crate with the same name as the package,  
- `src/lib.rs` as the root of the package’s library crate with the same name as the package.  

Cargo passes the crate root files to `rustc` to build the library or binary.  

If a package contains both `src/main.rs` and `src/lib.rs`, it has **two crates**:  
a **binary** and a **library**, both with the same name as the package.  

A package can have multiple binary crates by placing files inside `src/bin/`:  
Each file will be a separate binary crate.

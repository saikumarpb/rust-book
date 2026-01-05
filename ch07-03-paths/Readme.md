# Paths for Referring to an Item in the Module Tree

To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem.

To call a function, we need to know its path.

A path can take two forms:

- An absolute path is the full path starting from a crate root; 
    - for code from an external crate, the absolute path begins with the crate name
    - for code from the current crate, it starts with the literal `crate`.

- A relative path starts from the current module and uses self, super, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).

Our preference in general is to specify absolute paths because it’s more likely we’ll want to move code definitions and item calls independently of each other.

In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.

Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.
This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined.

Making the module public doesn’t make its contents public.

The pub keyword on a module only lets code in its ancestor modules refer to it, not access its inner code. Because modules are containers, there’s not much we can do by only making the module public; we need to go further and choose to make desired items within the module public.

The privacy rules apply to structs, enums, functions, and methods as well as modules.

### Best Practices for Packages with a Binary and a Library

    A Rust package can contain both a binary crate (src/main.rs) and a library crate (src/lib.rs). By default, both use the same package name.

    The binary crate should only have the minimal code needed to start the app. All core logic should live in the library crate. This allows other projects to reuse the library’s functionality.

    Define your module structure in src/lib.rs. Public functions or items from the library can be used in the binary by importing them using the package name.

    The binary crate acts like an external user of the library. it can only access the public API. This helps you design cleaner and better-structured code.

### Starting Relative Paths with super

In Rust, you can create a relative path that starts from the parent module by using super::.

It works like .. in a filesystem, which means go one folder up.

super:: lets you access items in the parent module, making your code more flexible and easier to refactor.
This is especially useful when a module is tightly related to its parent, and you want the freedom to move the parent module later without breaking references.

### Making Structs and Enums Public

In Rust, you can make structs and enums public using pub.

- When you mark a struct as pub, the struct becomes public, but its fields stay private by default.
- You can make fields public individually by adding pub before each field, depending on your needs.
- This gives you fine-grained control over what parts of the struct are exposed.

Enums marked as pub expose all their variants publicly by default.
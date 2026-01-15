# Error Handling in Rust

Rust groups errors into two categories: **recoverable** and **unrecoverable**.

Recoverable errors (e.g., file not found) can be reported and retried.  
Unrecoverable errors indicate bugs (e.g., out-of-bounds access) and should stop execution.

Most languages handle both using exceptions, but Rust does not have exceptions.  
Rust uses `Result<T, E>` for recoverable errors.  
Rust uses the `panic!` macro to halt execution on unrecoverable errors.

---

## Unrecoverable Errors with `panic!`

A panic can occur either by triggering one implicitly (e.g., indexing past array bounds)  
or by explicitly calling the `panic!` macro.

In both cases, the program enters a panic state.  
By default, Rust prints an error message, unwinds the stack, cleans up, and exits.

You can enable backtraces via an environment variable to see the call stack  
and help locate the source of the panic.

---

## Stack Unwinding vs Aborting

By default, when a panic occurs, Rust unwinds the stack.

Unwinding means Rust walks back up the call stack and cleans up data in each function.  
This cleanup process is safe but can be expensive.

Rust provides an alternative behavior: aborting immediately on panic.  
Aborting stops the program without running cleanup code.

Memory that the program was using will then need to be cleaned up by the operating system.  
Aborting is useful when your project needs to make the resultant binary as small as possible.

You can configure this behavior in `Cargo.toml` using profile settings:

```toml
[profile.release]
panic = "abort"
```

#### Example Panic

```rust
panic!("Crash and burn");
```

```rust
let vector = vec![1, 2, 3, 4];
vector[99];
```

----

### Comparison with C

In C, reading beyond the end of a data structure is undefined behavior.

The program may read whatever happens to be at that memory location,
even if the memory does not belong to the data structure.

This is known as a buffer overread.

Buffer overreads can lead to security vulnerabilities,
especially if an attacker can control the index to access sensitive data
stored after the intended data structure.

----
### Backtraces

Setting the RUST_BACKTRACE environment variable enables a backtrace.

A backtrace lists all function calls leading up to the error.

To read it:

- Start from the top and look for files you wrote

- That location is where the error originated

- Lines above are functions your code called

- Lines below are functions that called your code

These may include Rust core, standard library, or external crate code.

```bash
RUST_BACKTRACE=1 cargo run
```

---

### Debug Symbols

In order to get backtraces with this information, debug symbols must be enabled.

Debug symbols are enabled by default when using cargo build or cargo run
without the --release flag.
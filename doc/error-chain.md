# error-chain


## Initialize errors

Add `error-chain` initiatization code as [recommended](https://brson.github.io/2016/11/30/starting-with-error-chain):

```rust
// Simple and robust error handling with error-chain!

// `error_chain!` can recurse deeply, so limit it.
#![recursion_limit = "1024"]

// Import the macro. Be sure to add `error-chain` in your `Cargo.toml`.
#[macro_use]
extern crate error_chain;

// We put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// that `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

use errors::*;
```


## Catch errors

Our `main()` function uses the recommended code for `error_chain` handling:

```
fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        std::process::exit(1);
    }
}
```

Our `run()` function is where the work happens:

```
fn run() -> Result<()> {
    …
    Ok(())
}
```
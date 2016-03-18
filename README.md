# flu
flu is a framework for working with Lua 5.1 in the [Rust](https://www.rust-lang.org/) programming language. It's meant to provide quasi-safe abstractions over core Lua concepts – but also unsafe access to the Lua stack.

### What does it look like?
To interface with Lua through flu, you must first create a `flu::LuaContext`.You can do this by either wrapping around an existing `*mut lua_State`, or by using `flu::LuaContext::new()` to initialize a new one underneath.

```rust
let cxt = flu::LuaContext::new();
// or
let wrapped_cxt = flu::LuaContext::from_state(lua_state);
```

#### Abstractions
TODO

#### The stack
Modifying the stack is easy. `flu::LuaContext` has a function called `push`, which has the following definition:

```rust
pub fn push<T>(&self, val: T)
               where T: Push {
    val.push(self);
}
```

Essentially what this means is that everything that implements the `Push` trait can be pushed onto the stack. By default `Push` is implemented for the following types:
* `nil` (a unit struct)
* `bool`
* `i8`, `i16`, `i32`
* `f32`, `f64`
* `&str`, `String`
* `Option<T: Push>`
* `(A, B, ...) where A: Push, B: Push, ...`

This also goes the other way when reading from the stack. flu provides another trait called `Read` which allows for types that implement it to be read back to Rust (`Read` is also implemented for the types mentioned earlier).

`flu::LuaContext` has 2 methods for reading values back from the stack – `read` and `pop`. `read` will return the value from the stack, but **not** remove it, whereas `pop` will. A simple example of pushing a value to the stack then reading it back might look like this:

```rust
let cxt = flu::LuaContext::new();

cxt.push("hello world!");
let val = cxt.pop::<&str>();

assert_eq!(val, "hello world!");
```

### How do i get it?
flu is not currently on [crates.io](https://crates.io/), but you can use the latest version from GitHub by adding this to your `Cargo.toml`:

```toml
flu = { git = "https://github.com/fkaa/flu" }
```

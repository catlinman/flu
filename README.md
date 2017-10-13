
# flu #

flu is a framework for working with Lua 5.1 in the [Rust](https://www.rust-lang.org/) programming language. It's meant to provide quasi-safe abstractions over core Lua concepts – but also unsafe access to the Lua stack.

### What does it look like? ###
To interface with Lua through flu, you must first create a `flu::LuaContext`.You can do this by either wrapping around an existing `*mut lua_State`, or by using `flu::LuaContext::new()` to initialize a new one underneath.

```rust
let ctx = flu::LuaContext::new();
// or
let wrapped_ctx = flu::LuaContext::from_state(lua_state);
```

#### Abstractions ###

TODO

#### The stack ####

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
let ctx = flu::LuaContext::new();

ctx.push("hello world!");
let val = ctx.pop::<&str>();

assert_eq!(val, "hello world!");
```

## License ##

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution ###

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

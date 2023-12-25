## Learning Rust via the docs

- hello world every rust program starts with a ->

```rust
fn main(){
    // Code goes here
}
```

- you can handle rust project using the following commands

  - `cargo new <name>` to create a project
  - `cargo run` to run a project and build it
  - `cargo build` to build an executable of a project
  - `cargo check` to check for errors when compiling

- Variables

  - Variables are immutable by default or else specified other wise by the `mut` keyword

  ```rust
  let mut a= "i am mutable";
  let  a = "i am NOT mutable";
  ```

- Types
  - String is for dynamic string that could grow and the size i uknown

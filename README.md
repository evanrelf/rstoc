# rstoc

Table of contents for items in a Rust source file.

Currently an unpolished but functional prototype.

## Example

```rust
// demo.rs

const LAUNCH_NUKES: bool = false;

struct Person {
    name: String,
    age: u8,
}

enum Color {
    Red,
    Green,
    Blue,
}

fn is_even(n: usize) -> bool {
    n % 2 == 0
}
```

```
$ rstoc demo.rs
demo.rs:1:7:const LAUNCH_NUKES
demo.rs:3:8:struct Person
demo.rs:8:6:enum Color
demo.rs:14:4:fn is_even
```

Designed for scripting, e.g. fuzzy select and open in editor with cursor on
item:

```
$ rstoc demo.rs | fzf | cut -d : -f 1-3 | xargs "$EDITOR"
```

<!-- TODO: Test these examples with `cargo test` -->

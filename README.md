This derive macro creates a `new` associated function receiving as parameters the same fields declared in your `struct`.

So this:

```rust
#[derive(New)]
struct Cube {
	r: i32,
	c: i32,
	z: i32,
	w: i32,
}
```

... will create this:

```rust
impl Cube {
    fn new(r: i32, c: i32, z: i32, w: i32) -> Self {
        Self { r, c, z, w }
    }
}
```

It only works for basic types for now, see issues.

ps: Rust Analyzer already generates `new` function for you!

![Generate new](imgs/rust-analyzer-new-function.png)

https://rust-analyzer.github.io/manual.html#assists-code-actions

## Using it

https://crates.io/crates/new-derive

```
cargo add new-derive
```

```rust
use new_derive::New;

#[derive(New)]
struct Cube {
	r: i32,
	c: i32,
	z: i32,
	w: i32,
}

fn main() {
	let cube = Cube::new(1, 2, 3, 4);
	println!("{}", cube.r * cube.c * cube.z * cube.w);
}
```


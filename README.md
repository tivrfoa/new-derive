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

# derive_destructure

This crate allows you to destructure structs that implement `Drop`.

If you've ever struggled with error E0509
"cannot move out of type `T`, which implements the `Drop` trait"
then this crate may be for you.

Simply mark your struct with `#[derive(destructure)]`.
That gives it a `fn destructure`, which takes `self` by value and
turns that `self` into a tuple of the fields of the struct,
**without running the struct's `drop()` method**, like this:

```rust
let (field_1, field_2, ...) = my_struct.destructure();
```

For structs that don't implement `Drop` you don't really need this crate,
because Rust lets you destructure those kinds of structs very easily already.
You can simply move their fields out of them.
But for structs that do implement `Drop`, you can't simply move values out,
so there's no easy way to destructure them. That's why I made this crate.

Note: a tuple of 1 element in Rust is denoted as `(x,)`, not `(x)`.

## Example:
```rust
#[macro_use]
extern crate derive_destructure;

#[derive(destructure)]
struct ImplementsDrop {
    some_str: String,
    some_int: i32
}

impl Drop for ImplementsDrop {
    fn drop(&mut self) {
        panic!("We don't want to drop this");
    }
}

fn main() {
    let x = ImplementsDrop {
        some_str: "foo".to_owned(),
        some_int: 4
    };
    let (some_str, some_int) = x.destructure();
    // x won't get dropped now
}
```

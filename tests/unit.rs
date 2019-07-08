#[macro_use]
extern crate derive_destructure;

// This is perhaps rather pointless, as you can just use std::mem::forget instead...
// Oh well.

#[derive(destructure, remove_trait_impls)]
struct Foo;

impl Drop for Foo {
	fn drop(&mut self) {
		panic!("We don't want to drop this");
	}
}

#[derive(destructure, remove_trait_impls)]
struct Bar();

impl Drop for Bar {
	fn drop(&mut self) {
		panic!("We don't want to drop this");
	}
}

#[derive(destructure, remove_trait_impls)]
struct Baz {}

impl Drop for Baz {
	fn drop(&mut self) {
		panic!("We don't want to drop this");
	}
}

#[test]
fn test_foo_destructure() {
	let x = Foo;
	let () = x.destructure();
}

#[test]
fn test_foo_remove_trait_impls() {
	let x = Foo;
	let _ = x.remove_trait_impls();
}

#[test]
fn test_bar_destructure() {
	let x = Bar();
	let () = x.destructure();
}

#[test]
fn test_bar_remove_trait_impls() {
	let x = Bar();
	let _ = x.remove_trait_impls();
}

#[test]
fn test_baz_destructure() {
	let x = Baz{};
	let () = x.destructure();
}

#[test]
fn test_baz_remove_trait_impls() {
	let x = Baz{};
	let _ = x.remove_trait_impls();
}

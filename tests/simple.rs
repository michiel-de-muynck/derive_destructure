#[macro_use]
extern crate derive_destructure;

#[derive(destructure, remove_trait_impls)]
struct Foo {
	x: i64,
	y: f32
}

impl Drop for Foo {
	fn drop(&mut self) {
		panic!("We don't want to drop this");
	}
}

#[test]
fn test_simple_destructure() {
	let foo = Foo {
		x: 7,
		y: 8.9
	};
	let (x,y) = foo.destructure();
	assert_eq!(x, 7);
	assert_eq!(y, 8.9);
}

#[test]
fn test_simple_remove_trait_impls() {
	let foo = Foo {
		x: 7,
		y: 8.9
	};
	let foo = foo.remove_trait_impls();
	assert_eq!(foo.x, 7);
	assert_eq!(foo.y, 8.9);
}

#[macro_use]
extern crate derive_destructure;

#[derive(destructure)]
struct Foo {
	x: i64,
	y: f32
}

#[test]
fn test_simple() {
	let foo = Foo {
		x: 7,
		y: 8.9
	};
	let (x,y) = foo.destructure();
	assert_eq!(x, 7);
	assert_eq!(y, 8.9);
}

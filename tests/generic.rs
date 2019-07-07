#[macro_use]
extern crate derive_destructure;

#[derive(destructure)]
struct Foo<'a,'b,T> {
	some_ref: &'a i64,
	some_ref_mut: &'b mut i32,
	some_val: T
}

impl<'a,'b,T> Drop for Foo<'a,'b,T> {
	fn drop(&mut self) {
		panic!("Shouldn't drop this");
	}
}

#[test]
fn test_simple() {
	let x = 3;
	let mut y = 4;
	let foo = Foo {
		some_ref: &x,
		some_ref_mut: &mut y,
		some_val: 5.6
	};
	let (x,y,z) = foo.destructure();
	assert_eq!(*x, 3);
	assert_eq!(*y, 4);
	*y = 5;
	assert_eq!(*y, 5);
	assert_eq!(z, 5.6);
}

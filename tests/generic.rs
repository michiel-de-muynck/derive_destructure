#[macro_use]
extern crate derive_destructure;

#[derive(destructure, remove_trait_impls)]
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
fn test_simple_destructure() {
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

#[test]
fn test_simple_remove_trait_impls() {
	let x = 3;
	let mut y = 4;
	let foo = Foo {
		some_ref: &x,
		some_ref_mut: &mut y,
		some_val: 5.6
	};
	let foo = foo.remove_trait_impls();
	assert_eq!(*foo.some_ref, 3);
	assert_eq!(*foo.some_ref_mut, 4);
	*foo.some_ref_mut = 5;
	assert_eq!(*foo.some_ref_mut, 5);
	assert_eq!(foo.some_val, 5.6);
}

trait SomeTrait {}
impl SomeTrait for i32 {}

#[derive(destructure, remove_trait_impls)]
struct TraitBounded<T: SomeTrait>(T);

impl<T: SomeTrait> Drop for TraitBounded<T> {
	fn drop(&mut self) {
		panic!("Shouldn't drop this");
	}
}

#[derive(destructure, remove_trait_impls)]
struct TraitBounded2<'a,'b,T: SomeTrait+'b> where 'b: 'a {
	r: &'a &'b T
}

impl<'a,'b:'a,T: SomeTrait+'b> Drop for TraitBounded2<'a,'b,T> {
	fn drop(&mut self) {
		panic!("Shouldn't drop this");
	}
}

#[test]
fn test_trait_bounded_destructure() {
	let x = TraitBounded(7);
	let (x,) = x.destructure();
	assert_eq!(x, 7);
}

#[test]
fn test_trait_bounded_remove_trait_impls() {
	let x = TraitBounded(7);
	let x = x.remove_trait_impls();
	assert_eq!(x.0, 7);
}

#[test]
fn test_trait_bounded2_destructure() {
	let i = 7;
	let x = TraitBounded2{
		r: &&i
	};
	let (r,) = x.destructure();
	assert_eq!(**r, 7);
}

#[test]
fn test_trait_bounded2_remove_trait_impls() {
	let i = 7;
	let x = TraitBounded2{
		r: &&i
	};
	let x = x.remove_trait_impls();
	assert_eq!(**x.r, 7);
}

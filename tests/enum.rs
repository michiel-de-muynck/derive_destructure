#[macro_use]
extern crate derive_destructure;

#[derive(remove_trait_impls)]
pub enum Simple {
	A,
	B,
	C
}

impl Drop for Simple {
	fn drop(&mut self) {
		panic!("We shouldn't drop this!");
	}
}

#[test]
fn test_simple() {
	let simple = Simple::A;
	let simple = simple.remove_trait_impls();
	if let SimpleWithoutTraitImpls::A = simple {
	} else {
		panic!();
	}
}

pub trait SomeTrait {}
impl SomeTrait for i32 {}

#[derive(remove_trait_impls)]
pub enum GenericEnum<T: SomeTrait> {
	A(T),
	B(T,f64),
	C {
		x: String,
		y: T
	},
	D
}

impl<T:SomeTrait> Drop for GenericEnum<T> {
	fn drop(&mut self) {
		panic!("We shouldn't drop this!");
	}
}

#[test]
fn test_generic_a() {
	let e = GenericEnum::A(5);
	let e = e.remove_trait_impls();
	if let GenericEnumWithoutTraitImpls::A(x) = e {
		assert_eq!(x, 5);
	} else {
		panic!();
	}
}

#[test]
fn test_generic_b() {
	let e = GenericEnum::B(7,8.9);
	let e = e.remove_trait_impls();
	if let GenericEnumWithoutTraitImpls::B(x,y) = e {
		assert_eq!(x, 7);
		assert_eq!(y, 8.9);
	} else {
		panic!();
	}
}

#[test]
fn test_generic_c() {
	let e = GenericEnum::C {
		x: "foo".to_owned(),
		y: 7
	};
	let e = e.remove_trait_impls();
	if let GenericEnumWithoutTraitImpls::C{x,y} = e {
		assert_eq!(x, "foo");
		assert_eq!(y, 7);
	} else {
		panic!();
	}
}

#[test]
fn test_generic_d() {
	let e = GenericEnum::<i32>::D;
	let e = e.remove_trait_impls();
	if let GenericEnumWithoutTraitImpls::D = e {
	} else {
		panic!();
	}
}

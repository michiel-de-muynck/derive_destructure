#[macro_use]
extern crate derive_destructure;

use std::rc::Rc;
use std::cell::Cell;

#[derive(destructure)]
struct DropChecker(Rc<Cell<bool>>);

impl DropChecker {
	fn new() -> Self {
		DropChecker(Rc::new(Cell::new(false)))
	}
}

impl Drop for DropChecker {
	fn drop(&mut self) {
		let dropped = &self.0;
		if dropped.get() {
			panic!("Dropped twice!");
		}
		dropped.set(true)
	}
}

#[test]
fn test_droptest_normal_drop() {
	let drop_checker = DropChecker::new();
	let dropped_rc_clone = Rc::clone(&drop_checker.0);
	assert_eq!(dropped_rc_clone.get(), false);
	drop(drop_checker);
	assert_eq!(dropped_rc_clone.get(), true);
}

#[test]
fn test_droptest_destructure() {
	let drop_checker = DropChecker::new();
	let dropped_rc_clone = Rc::clone(&drop_checker.0);
	assert_eq!(dropped_rc_clone.get(), false);
	let (dropped_rc,) = drop_checker.destructure();
	assert_eq!(dropped_rc.get(), false);
	assert_eq!(dropped_rc_clone.get(), false);
}

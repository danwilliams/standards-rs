#![allow(non_snake_case)]
#[cfg(test)]

//		Packages

use super::*;
use crate::tests;



//		Tests

//		Foo																		

//		new																		
#[test]
fn new__set_id() {
	let foo = Foo::new(
		Some(42),
		Settings::default(),
	);
	assert_eq!(foo.id,             Some(42));
	assert_eq!(foo.settings.Style, Style::Standard);
}
#[test]
fn new__no_id() {
	let foo = Foo::new(
		None,
		Settings::default(),
	);
	assert!(foo.id.is_none());
	assert_eq!(foo.settings.Style, Style::Standard);
}



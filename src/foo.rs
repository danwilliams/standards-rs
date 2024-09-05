//! The foo module.
#![allow(non_snake_case, reason = "The Settings struct uses PascalCase")]



//		Modules

#[cfg(test)]
#[path = "tests/foo.rs"]
mod tests;



//		Packages

use core::{
	error::Error,
	fmt::{Display, self},
};



//		Enums

//		Style																	
/// The style of a foo.
/// 
/// The default style is [`Standard`](Style::Standard).
/// 
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Style {
	/// The ordinary style of foo.
	#[default]
	Standard,
	
	/// The Bar style.
	Bar,
}

//		FooError																
/// The possible errors that can occur when working with a foo.
#[expect(dead_code, reason = "This is an example error and is not actually used")]
#[derive(Debug)]
pub enum FooError {
	/// The foo is invalid.
	Invalid,
}

impl Display for FooError {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let description = match *self {
			Self::Invalid => "Invalid foo".to_owned(),
		};
		write!(f, "{description}")
	}
}

impl Error for FooError {}



//		Structs

//		Settings																
/// The various settings available to configure a foo.
#[derive(Debug, Default)]
pub struct Settings {
	/// The style of the foo.
	pub Style: Style,
}

//		Foo																	
/// An instance of a foo.
/// 
/// # See Also
/// 
/// * [`Bar`](crate::bar::Bar)
/// 
#[derive(Default)]
pub struct Foo {
	//		Public properties													
	/// The unique id of the foo. This is `None` if it is new and unsaved.
	pub id:   Option<u64>,
	
	//		Private properties													
	/// The foo settings.
	settings: Settings,
}

impl Foo {
	//		new																	
	/// Creates a new instance of a foo.
	/// 
	/// # Parameters
	/// 
	/// * `id`       - The unique id, if there is one.
	/// * `settings` - The settings to use for the foo.
	/// 
	#[expect(clippy::missing_const_for_fn, reason = "This will do more in future")]
	pub fn new(
		id:       Option<u64>,
		settings: Settings,
	) -> Self {
		Self {
			id,
			settings,
		}
	}
	
	//		settings															
	/// Gets a read-only reference to the foo's settings.
	pub const fn settings(&self) -> &Settings {
		&self.settings
	}
}



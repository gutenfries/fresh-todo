#[macro_use]
/*
 because of the weird conventions around using a systems language for the web, there are
 several macros to make that easier. put all macros you end up writing here.
*/

pub mod macros {
	use crate::wasm_bindgen::prelude::*;

	#[wasm_bindgen]
	extern "C" {
		#[wasm_bindgen(js_namespace = console)]
		/// wrapper for the ```console.log()``` javascript api
		pub fn log(s: &str);
	}

	#[macro_export]

	/// prints a formatted string to the console (stdout equivalent)
	/// first argument is a string literal (ex ```"Hi {}"```).
	///
	/// all of the remaining arguments are  passed into the string literal
	///
	/// # Example
	/// ```
	/// printf!("Hi {}", "Bob");
	/// ```
	/// output: `Hi Bob`
	macro_rules! printf {
		// case 1: 1 argument; only a raw string is passed
		($a:expr) => {
			crate::macros::macros::log($a);
		};
		// case 2: 2 arguments; a string literal and a raw variable is passed
		($a:expr, $b:expr$(,)?) => {
			crate::macros::macros::log(&format!($a, $b));
		};
		// case 3: n arguments; a string literal and n raw variables are passed
		($a:expr, $($b:expr$(,)?),+) => {
		crate::macros::macros::log(&format!($a, $($b),+));
	};
}
}

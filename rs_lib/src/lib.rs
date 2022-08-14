extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
mod macros;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
	return a + b;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(1, 2);
		assert_eq!(result, 3);
	}
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	printf!("Hello from {}!", name); // should output "Hello from Rust!"
}

#[macro_use]
extern crate curryrs;

use curryrs::types::*;

safe_ffi! (

	fn double(x: I32) -> I32 {
		2 * x
	}

	fn square(x: U64) -> U64 {
		x * x
	}

	fn cube(x: I64) -> I64 {
		x * x * x
	}

);

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

// Not great to ignore, but we'd otherwise get many hundreds of warnings.
#![allow(improper_ctypes)]

mod inner {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod basisu {
	pub use crate::inner::root::basisu::*;
}
pub mod basist {
	pub use crate::inner::root::basisu::*;
}

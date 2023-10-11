#![doc = include_str!("../README.md")]
#![deny(missing_docs, unsafe_code)]

mod debug_trait_only_req;

pub use colored::Color; // Re-export as it is part of the public API to call functions in this lib
pub use debug_trait_only_req::{eq_on_debug, print_second_if_different};

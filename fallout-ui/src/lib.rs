#![forbid(unsafe_code)]
#![recursion_limit = "1024"]

#[cfg(not(target_arch = "wasm32"))]
compile_error!(
    "rustc is not correctly configured for this crate, target target_arch has to be wasm32"
);

#[macro_use]
extern crate lazy_static;

pub mod components;
pub mod hooks;
pub mod stories;
pub mod utils;

#![feature(rustc_private)]

extern crate rustc_ast_pretty;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_middle;
extern crate rustc_attr;

mod adapter;
mod util;
mod compiler_config;

pub use adapter::{Adapter, Vertex};
pub use util::get_sysroot;
pub use compiler_config::CompilerConfig;

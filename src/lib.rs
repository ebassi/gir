#![cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
#![cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#![cfg_attr(feature = "cargo-clippy", allow(write_literal))]

extern crate git2;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;
extern crate stripper_lib;
extern crate toml;
extern crate xml;

/// Log warning only if type in generated library
macro_rules! warn_main {
    ($tid: expr, $target:expr, $($arg:tt)*) => (
        if $tid.ns_id == ::library::MAIN_NAMESPACE {
            warn!($target, $($arg)*);
        }
    );
}

//generated by build.rs
mod gir_version;

mod analysis;
mod case;
mod chunk;
mod codegen;
mod config;
mod consts;
mod custom_type_glib_priority;
mod env;
mod file_saver;
mod git;
mod library;
mod library_postprocessing;
mod library_preprocessing;
mod nameutil;
mod parser;
mod traits;
pub mod update_version;
mod version;
mod visitors;
mod writer;
mod xmlparser;

pub use analysis::class_hierarchy::run as class_hierarchy_run;
pub use analysis::namespaces::run as namespaces_run;
pub use analysis::run as analysis_run;
pub use analysis::symbols::run as symbols_run;
pub use codegen::generate as codegen_generate;
pub use config::{Config, WorkMode};
pub use env::Env;
pub use library::Library;

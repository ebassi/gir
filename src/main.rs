extern crate case;
extern crate docopt;
extern crate env_logger;
extern crate git2;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate xml;
extern crate toml;

use std::error::Error;
use std::process;

use env::Env;
use library::Library;

//generated by build.rs
mod gir_version;

mod analysis;
mod codegen;
mod config;
mod env;
mod file_saver;
mod git;
mod gobjects;
mod library;
mod nameutil;
mod parser;
mod traits;
mod version;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    if let Err(err) = do_main() {
        println!("{}", err);
        process::exit(1);
    }
}

fn do_main() -> Result<(), Box<Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "gir=warn");
    }
    try!(env_logger::init());

    let cfg = match config::Config::new() {
        Ok(cfg) => cfg,
        Err(config::Error::CommandLine(ref err)) if !err.fatal() => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    };

    let mut library = Library::new(&cfg.library_name);
    library.read_file(&cfg.girs_dir, &cfg.library_full_name());
    library.fill_in();

    let env = Env{ library: library, config: cfg };
    codegen::generate(&env);

    Ok(())
}

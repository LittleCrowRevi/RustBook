use std::env;
use std::fs;
use std::process;
use std::error::Error;

use rust_book::{Config};

pub fn minigrep() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments!: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_book::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }


}

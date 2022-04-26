use std::{env, process};
use minigrep_iter::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_iter::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

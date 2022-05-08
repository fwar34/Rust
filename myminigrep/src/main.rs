use std::{env, process};

use myminigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        println!("{}", err);
        process::exit(1);
    });

    if let Err(e) = myminigrep::run(&config) {
        println!("{}", e);
        process::exit(1);
    }
}

use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    // let config = match Config::new(&args) {
    //     Ok(config) => config,
    //     Err(e) => {
    //         println!("{}", e);
    //         return;
    //     }
    // };
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem occur: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}\n----------------------", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

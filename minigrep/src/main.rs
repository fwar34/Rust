use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    run(config);
}

struct Config {
    query: String,
    filename: String,
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("something went wrong reading the file");
    
    println!("With text:\n{}", contents);
}

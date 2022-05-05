use std::env::var;
use std::env::Args;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub sensitivity: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        Ok(Config {
            query: match args.next() {
                Some(query) => query,
                None => return Err("not query"),
            },
            filename: match args.next() {
                Some(filename) => filename,
                None => return Err("not filename"),
            },
            sensitivity: var("SENSITIVITY").is_ok(),
        })
    }
}

pub fn fun(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let results = if config.sensitivity {
        search_sensitive(&config.query, &contents);
    } else {
        search(&config.query, &contents);
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a String) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

fn search_sensitive<'a>(query: &str, contents: &'a String) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

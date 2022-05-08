use std::{
    collections::HashMap,
    env::args,
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    let mut args = args();
    args.next();
    let mut word_map: HashMap<&str, u32> = HashMap::new();
    let contents = fs::read_to_string(args.next().unwrap()).unwrap();
    let vs: Vec<&str> = contents.split(' ').collect();
    for s in vs {
        let s = s.trim();
        if let Some(&count) = word_map.get(&s) {
            word_map.insert(s, count + 1);
        } else {
            word_map.insert(s, 1);
        }
    }

    for (k, v) in word_map {
        println!("{}:{}", k, v);
    }

    println!("Hello, world!");
}

fn file_word(path: &str, word_map: &mut HashMap<&str, u32>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let words: Vec<&str> = line.split(' ').collect();
            for word in words {
                let word = word.trim();
                if let Some(&count) = word_map.get(word) {
                    word_map.insert(word, count + 1);
                }
            }
        }
    }
}

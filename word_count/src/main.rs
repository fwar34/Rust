use std::{
    collections::HashMap,
    env::args,
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    let mut args = args();
    args.next();
    let path = args.next().expect("path is empty!");
    let mut word_map: HashMap<String, u32> = HashMap::new();
    let contents = fs::read_to_string(&path).unwrap();
    let vs = contents.split_whitespace();
    vs.for_each(|s| {
        let s = s.trim();
        *word_map.entry(s.to_string()).or_insert(0) += 1;
    });

    word_map.iter().for_each(|(k, v)| {
        println!("{}:{}", k, v);
    });

    println!("Hello, world!");

    let mut map2 = HashMap::new();
    use_buf(&path, &mut map2);
}

#[allow(unused)]
fn use_buf(path: &str, map: &mut HashMap<String, u32>) {
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);
    reader.lines().for_each(|line| {
        line.unwrap().split_whitespace().for_each(|s| {
            *map.entry(s.trim().to_string()).or_insert(0) += 1;
        });
    });

    map.iter().for_each(|(k, v)| {
        println!("{}:{}", k, v);
    })
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    #[test]
    fn test_insert() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        map.entry("key").or_insert(1);
        assert_eq!(*map.get("key").unwrap(), 1);
        *map.entry("key").or_insert(1) += 1;
        assert_eq!(*map.get("key").unwrap(), 2);
    }
}

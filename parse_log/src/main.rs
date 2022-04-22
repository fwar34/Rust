// #[macro_use]
// extern crate lazy_static;
// use lazy_static::lazy_static;
use rand::Rng;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write, Lines};
use std::cmp::Ordering;
use std::path::Path;
use std::fs::File;
use regex::Regex;

#[allow(dead_code)]
fn test_file1() {
    let path = Path::new("test.txt");
    let display = path.display();

    if let Ok(mut file) = File::open(path) {
        println!("file {:?}", file);
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("cloudn't read {}: {}", display, why),
            Ok(_) => println!("{} contains:\n{}", display, s),
        }
    } else {
        panic!("cloudn't open {}", display)
    }
}

#[allow(dead_code)]
fn test_file2() {
    let path = Path::new("test.txt");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("cloudn't open {}: {}", display, why),
        Ok(file) => file,
    };
    println!("file {:?}", file);

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("cloudn't read{}: {}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }
}

fn test_file3() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

#[allow(dead_code)]
fn test_file4() -> std::io::Result<()> {
    let file = File::create("test.txt").unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(b"sdlkfsldkfj xxx")?;
    writer.flush()?;
    Ok(())
}

#[allow(dead_code)]
fn test_regex1() {
    let s = "123-4567-89,987-6543-21";
    let r = Regex::new(r"\d{3}-(\d{4}).*").unwrap();
    if r.is_match(s) {
        println!("found!");
    }

    for cap in r.captures_iter(s) {
        println!("found group {}", &cap[1]);
    }
}

#[allow(dead_code)]
fn guess() {
    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("Please type a number:");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        let guess: u32 = guess.trim().parse().expect("Please type a num!");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

const LOG: &str = "/home/feng/bug/1157238220.log.1";
const USERID: &str = "/home/feng/bug/userid.txt";
const RET: &str = "/home/feng/bug/ret.txt";

fn find_exit(userid: String, lines: Lines<BufReader<File>> , ret: BufWriter<File>) -> bool {
    let re: Regex = Regex::new(&(r"(\d{2}:\d{2}:\d{2}).*BMS user exit conference success.*".to_string() + &userid)).unwrap();

    for line in lines {
        for cap in re.captures_iter(&line.unwrap()) {
            if cap.len() != 0 {

                return true;
            }
        }
    }

    return false;
}

fn parse() {
    let file_userid = File::open(USERID).unwrap();
    let reader_userid = BufReader::new(file_userid);
    let file_log = File::open(LOG).unwrap();
    let reader_log = BufReader::new(file_log);
    let file_ret = File::create(RET).unwrap();
    let mut writer_ret = BufWriter::new(file_ret);

    let lines = reader_log.lines();

    for (index, line) in reader_userid.lines().enumerate() {
        let line = line.unwrap();
        println!("{}:{}", index + 1, line);
        if find_exit(line, lines, &mut writer_ret) { // 找到退会的直接写到 ret.txt

        } else { // 找不到退会的就查找下线的

        }
    }
}

fn main() {
    test_regex1();
    // test_file1();
    // test_file2();
    test_file3();
    let _ = test_file4();
    // guess();
    // parse();
}

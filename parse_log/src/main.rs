use rand::Rng;
use std::io::{self, Read};
use std::cmp::Ordering;
use std::path::Path;
use std::fs::File;

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

fn main() {
    test_file1();
    test_file2();
    guess();
}

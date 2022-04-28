struct People {
    name: String,
    age: u32,
}

#[allow(dead_code)]
enum Family {
    Address(String),
    Person(People),
    Color(i32, i32, i32)
}

fn main() {
    let family = Family::Person(People{name: String::from("Xiaoming"), age: 5});
    match family {
        Family::Person(People{name, age: age @ 0..=10}) => println!("name = {}, age = {}", name, age),
        Family::Person(People{name, age: age @ (12 | 23 | 11)}) if name == "Lilei" => println!("{}", age),
        Family::Color(a, b, c) if a > 3 && b < 240 => println!("{} {} {}", a, b, c),
        _ => {}
    }
    println!("Hello, world!");
}

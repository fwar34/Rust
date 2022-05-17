fn main() {
    println!("Hello, world!");
    println!("{}", find_file_extension("main.rs").unwrap());
    println!("{}", find_file_extension2("main.rs").unwrap());
}

fn find_file_extension(file: &str) -> Option<&str> {
    match file.find('.') {
        Some(i) => Some(&file[i + 1..]),
        None => None,
    }
}

fn find_file_extension2(file: &str) -> Option<&str> {
    file.find('.').map(|i| &file[i + 1..])
}

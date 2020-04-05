fn main() {
    println!("Hello, world!");
}

struct Option {
    pub some_int: i32,
    pub some_string: String,
}

impl Option {
    pub fn new<T: Into<String>>(some_int: i32, some_string: T) -> Option {
        Option {
            some_int: some_int,
            some_string: some_string.into(),
        }
    }
}

struct Service<'a> {
    option: &'a Option,
}

impl<'a> Service<'a> {
    pub fn new(option: &'a Option) -> Service {
        Service { option: option }
    }

    pub fn name(&self) -> String {
        self.option.some_string.clone()
    }
}

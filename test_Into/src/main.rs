// https://www.jianshu.com/p/be4f12c9ac83
fn main() {
    println!("Hello, world!");
}

struct Config {
    path: String,
}

impl Config {
    fn new(path: String) -> Config {
        Config { path }
    }

    fn get_path(self) -> String {
        self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let path = "./Cargo.toml";
        let config = Config::new(path.to_string());
        assert_eq!(path, config.get_path())
    }

    #[test]
    fn test_config_into() {
        let path = "./Cargo.toml";
        let config = ConfigInto::new(path);
        assert_eq!(path, config.get_path());

        let path = "./Cargo.toml";
        let config = ConfigInto::new(path.to_string());
        assert_eq!(path, config.get_path())
    }
}

struct ConfigInto {
    path: String,
}

impl ConfigInto {
    fn new<T: Into<String>>(path: T) -> Config {
        Config { path: path.into() }
    }

    fn get_path(self) -> String {
        self.path
    }
}

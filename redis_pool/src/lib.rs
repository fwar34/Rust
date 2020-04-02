extern crate redis;
use redis::Commands;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn do_something() -> redis::RedisResult<()> {
        let client = redis::Client::open("redis://192.168.125.34/")?;
        let mut con = client.get_connection()?;

        let _: () = con.set("my_key", 42)?;
        con.get("my_key")
    }
}

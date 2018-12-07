pub mod network; 
pub mod client;

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        //super::client::connect();
        client::connect();
    }
}

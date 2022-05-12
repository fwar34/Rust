use std::{
    env::{args, Args},
    process,
};

struct Config<'a> {
    ip: &'a str,
    port: u32,
}

fn parse_arguments<'a>(mut args: Args) -> Result<Config<'a>, &'static str> {
    args.next();
    let ip_port = args.next().expect("Please use 'exe ip:port' to start");
    let ip_port: Vec<&str> = ip_port.split(':').collect();
    if ip_port.len() != 2 {
        eprintln!("Please use 'exe ip:port' to start");
        process::exit(-1);
    } else {
        Ok(Config{
            ip: ip_port[0],
            port: ip_port[1].parse().unwrap(),
        })
    }
}

fn main() {
    let args = args();
    parse_arguments(args);
}

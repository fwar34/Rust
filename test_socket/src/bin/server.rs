use std::{
    env::{args, Args},
    io::Read,
    net::TcpListener,
};

#[allow(unused)]
struct Config {
    ip: String,
    port: u16,
}

#[allow(unused)]
fn parse_arguments(mut args: Args) -> Result<Config, &'static str> {
    args.next();
    let ip_port = args.next().expect("Please use 'exe ip:port' to start");
    let ip_port: Vec<&str> = ip_port.split(':').collect();
    if ip_port.len() != 2 {
        return Err("Please use 'exe ip:port' to start");
    } else {
        Ok(Config {
            ip: ip_port[0].to_string(),
            port: ip_port[1].parse().unwrap(),
        })
    }
}

#[allow(unused)]
fn convert_ip(ip: &String) -> Result<Vec<u8>, &'static str> {
    let ip_vec: Vec<&str> = ip.split('.').collect();
    if ip_vec.len() != 4 {
        return Err("Please use format xx.xx.xx.xx in ip");
    }
    let mut ret_vec: Vec<u8> = Vec::new();
    ip_vec.iter().for_each(|element| {
        ret_vec.push(element.parse().unwrap());
    });
    Ok(ret_vec)
}

fn start_server(mut args: Args) -> Result<(), &'static str> {
    // let ip_vec = convert_ip(&config.ip)?;
    // let ip = config.ip.parse::<Ipv4Addr>().expect("Please use format xx.xx.xx.xx in ip");
    // let socket = SocketAddrV4::new(ip, config.port);
    args.next().unwrap(); // skip exe name
                          // let socket: SocketAddrV4 = args
                          //     .next()
                          //     .expect("Please use 'exe ip:port' to start")
                          //     .parse()
                          //     .expect("Please use format xx.xx.xx.xx in ip");
                          // let listener = TcpListener::bind(socket).expect("Create listener failed");
    let listener = TcpListener::bind(args.next().unwrap_or("127.0.0.1:9999".to_string())).unwrap();
    println!("Start listen on {}:{}", socket.ip(), socket.port());
    let (mut tcp_stream, addr) = listener.accept().unwrap();
    println!("Receive connect from {:?}", addr);
    let mut read_buf = Vec::new();
    let read_length = tcp_stream.read_to_end(&mut read_buf).unwrap();
    println!("Read {} bytes from {:?}", read_length, addr);
    Ok(())
}

fn main() -> Result<(), &'static str> {
    // let config = parse_arguments(args).unwrap();
    start_server(args())?;
    Ok(())
}

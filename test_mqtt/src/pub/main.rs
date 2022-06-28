// https://zhuanlan.zhihu.com/p/264181114
use std::{env, process, time::Duration};

use mqtt::{connect_options, create_options};

extern crate paho_mqtt as mqtt;

const DFLT_BROKER: &str = "tcp://broker.emqx.io:1883";
const DFLT_CLIENT: &str = "rust_publish";
const DFLT_TOPICS: &[&str] = &["rust/mqtt", "rust/test"];

const QOS: i32 = 1;

fn main() {
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| DFLT_BROKER.to_string());

    let create_options = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(DFLT_CLIENT.to_string())
        .finalize();

    let cli = mqtt::Client::new(create_options).unwrap_or_else(|err| {
        println!("Error creating client: {:?}", err);
        process::exit(1);
    });

    let connect_options = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    if let Err(e) = cli.connect(connect_options) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    for num in 0..5 {
        let content = "hello world!".to_string() + &num.to_string();
    }
}

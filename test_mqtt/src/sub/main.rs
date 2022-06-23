use paho_mqtt as mqtt;
use std::{env, process};

const DFLT_BROKER: &str = "tcp://broker.emqx.io:1883";
const DFLT_CLIENT: &str = "rust_subscribe";
const DFLT_TOPIC: &[&str] = &["rust/mqtt", "rust/test"];

const DFLT_QOS: &[i32] = &[0, 1];

fn main() {
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| DFLT_BROKER.to_string());

    let create_opt = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(DFLT_CLIENT.to_string())
        .finalize();

    let mut client = mqtt::Client::new(create_opt).unwrap_or_else(|err| {
        println!("Error creating the client {:?}", err);
        process::exit(1);
    });
}

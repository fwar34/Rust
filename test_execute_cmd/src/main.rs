// https://rustcc.gitbooks.io/rustprimer/content/std/process.html
use std::{env::args, process::Command};

fn main() {
    let mut args = args();
    args.next().unwrap();
    let pattern = args.next().unwrap_or("main".to_string());
    let path = args.next().unwrap_or("./".to_string());
    let output = Command::new("grep")
        .arg("-n")
        .arg("-r")
        .arg(&pattern)
        .arg(&path)
        .output()
        .unwrap_or_else(|e| panic!("panic error:{}", e));
    let st = String::from_utf8_lossy(&output.stdout);
    println!("output:");
    for line in st.split('\n') {
        println!("{}", line);
    }
}

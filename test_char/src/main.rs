fn main() {
    let love = '❤'; // char支持unicode，一个char4个字节
    let byte: u8 = b'A';
    println!("Hello, world! {}, {}", love, byte);
}

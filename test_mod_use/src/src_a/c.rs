use crate::src_a::a::*; // 绝对路径

pub fn c_echo() {
    println!("c_echo");
    a_echo();
}

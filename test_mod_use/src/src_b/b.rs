use crate::src_a::a_echo; // 绝对路径

pub fn b_echo() {
    println!("b_echo! => call a()!");
    a_echo();
}

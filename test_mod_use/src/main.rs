// 那么，Rust 的多层模块遵循如下两条规则：
// 1. 优先查找xxx.rs 文件
//  1) main.rs、lib.rs、mod.rs中的mod xxx; 默认优先查找同级目录下的 xxx.rs 文件；
//  2) 其他文件yyy.rs中的mod xxx;默认优先查找同级目录的yyy目录下的 xxx.rs 文件；
// 2. 如果 xxx.rs 不存在，则查找 xxx/mod.rs 文件，即 xxx 目录下的 mod.rs 文件。

// 优先查找同级的 src_a.rs 文件，符合 1-1)
pub mod src_a;
pub use self::src_a::*;
// 优先查找同级的 src_b.rs 文件，符合 1-1)
pub mod src_b;
pub use crate::src_b::*;

fn main() {
    println!("Hello world!");
    src_a::a_echo();
    src_b::b_echo();
}

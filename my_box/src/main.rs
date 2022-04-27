use std::{fmt::Display, ops::Deref};

struct MBox<T: Display>(T);

impl<T> MBox<T>
where
    T: Display,
{
    fn new(t: T) -> Self {
        MBox(t)
    }
}

impl<T> Deref for MBox<T>
where
    T: Display,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MBox<T>
where
    T: Display,
{
    fn drop(&mut self) {
        println!("MBox drop {}", self.0);
    }
}

fn main() {
    let test = MBox::new(55);
    let test2 = MBox::new(66);
    drop(test); // 使用 std::mem::drop 函数主动调用 test 的 drop 析构函数
    println!("MBox test contains: {}", *test2);
}

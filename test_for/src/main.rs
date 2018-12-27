//https://blog.csdn.net/guiqulaxi920/article/details/78823541

fn main() {
    for i in (0..10).filter(|x| x % 2 == 0) {
        println!("{}", i);
    }

    println!("test....");

    for i in (1..5 + 1).rev() {
        println!("{}", i);
    }
}

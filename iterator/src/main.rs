//https://kaisery.github.io/trpl-zh-cn/ch13-02-iterators.html#a%E4%BD%BF%E7%94%A8%E9%97%AD%E5%8C%85%E8%8E%B7%E5%8F%96%E7%8E%AF%E5%A2%83
fn main() {
    println!("Hello, world!");
    calling_next_directly();
    using_other_iterator_trait_methods();
}

//一个1到5的迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

//出于某种原因我们希望获取 Counter 实例产生的值，将这些值与另一个 Counter
//实例在省略了第一个值之后产生的值配对，将每一对值相乘，只保留那些可以被三整除的结果，然后将所有保留的结果相加，这可以如示例
//13-23 中的测试这样做：
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(sum, 18);
}

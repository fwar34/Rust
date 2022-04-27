#[derive(Debug)]
enum List<'a> {
    Cons(i32, Box<&'a List<'a>>),
    Nil,
}

use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    let tmp = Cons(10, Box::new(&Nil));
    let a = Cons(5, Box::new(&tmp));
    let b = Cons(3, Box::new(&a));
    let c = Cons(4, Box::new(&a));
    println!("a: {:?} b: {:?} c: {:?}", a, b, c);
    test_list2();
}

#[derive(Debug)]
enum List2<T> {
    Cons2(T, Rc<List2<T>>),
    Nil2,
}

use crate::List2::{Cons2, Nil2};
fn test_list2() {
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    let b = Cons2(3, Rc::clone(&a));
    let c = Cons2(4, Rc::clone(&a));
    println!("a: {:?} b: {:?} c: {:?}", a, b, c);
}

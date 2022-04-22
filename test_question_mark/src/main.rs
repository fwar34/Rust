// https://blog.csdn.net/wowotuo/article/details/78828298
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let ret = read_username_from_file();
    match ret {
        Err(e) => println!("{:?}", e),
        Ok(_) => println!("read ok"),
    };

    let _ = read_username_from_file2();
    ///////////////////
    not_use_question_mark();
    let _ = use_question_mark();
}

// 没有使用操作符`?`
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 使用操作符`?`
#[allow(dead_code)]
fn read_username_from_file2() -> Result<String, io::Error> {
    // 对象本身返回类型就是Result; 且在Result类函数体内
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// ===============================================================
#[allow(dead_code)]
struct Info {
    name: String,
    age: i32,
    rating: i32,
}

// 不使用`?`
#[allow(dead_code)]
fn write_info(info: &Info) -> io::Result<()> {
    let mut file = match File::create("my_best_friends.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    // Early return on error
    if let Err(e) = file.write_all(format!("name: {}\n", info.name).as_bytes()) {
        return Err(e);
    }
    if let Err(e) = file.write_all(format!("age: {}\n", info.age).as_bytes()) {
        return Err(e);
    }
    if let Err(e) = file.write_all(format!("rating: {}\n", info.rating).as_bytes()) {
        return Err(e);
    }
    Ok(())
}

// 有了`?`操作符，可以改成下面：
#[allow(dead_code)]
fn write_info2(info: &Info) -> io::Result<()> {
    let mut file = File::create("my_best_friends.txt")?;
    // Early return on error
    file.write_all(format!("name: {}\n", info.name).as_bytes())?;
    file.write_all(format!("age: {}\n", info.age).as_bytes())?;
    file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;
    Ok(())
}

// ==============================================================
#[allow(dead_code)]
fn test_io(s: &str) -> io::Result<()> {
    let mut f = File::create(s)?;
    f.write_all(b"Hello, world!")?;
    f.sync_data()?;
    Ok(())
}


// https://www.jianshu.com/p/46872e6bffce
// 什么是问号操作符?
// 参考: https://doc.rust-lang.org/book/second-edition/ch09-02-recoverable-errors-with-result.html
// 参考: https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about


// 由于Rust中没有Exception异常处理的语法,
// Rust只有panic报错, 并且panic不允许被保护, 因为没有提供 try 这种语法.
// Rust的异常处理是通过 Result 的 Ok 和 Err 成员来传递和包裹错误信息.
// 然而错误信息的处理一般都是要通过match来对类型进行比较, 所以很多时候
// 代码比较冗余, 通过?符号来简化Ok和Err的判断.

// 下面的例子提供了一个不使用?符号 以及 一个使用?符号的样例代码.
fn halves_if_even<'a>(i: i32) -> Result<i32, &'a str> {
    if i % 2 == 0 {
        Ok(i / 2)
    } else {
        Err("error")
    }
}

fn not_use_question_mark() {
    let a = 10;
    let half = halves_if_even(a);
    let half = match half {
        Ok(item) => item,
        Err(e) => panic!("{}", e),
    };
    assert_eq!(half, 5);
}

fn use_question_mark<'a>() -> Result<i32, &'a str> { // 这里必须要返回 Result，因为问号返回的就是 Result
    let a = 10;
    let half = halves_if_even(a)?; // 因为?要求其所在的函数必须要返回 Result
    println!("half = {}", half); // a 为奇数不会执行到这行，会从上面的一行问号处返回，偶数执行到这行
    // return Ok(half);
    Ok(half)
}

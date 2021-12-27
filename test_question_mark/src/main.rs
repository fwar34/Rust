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
fn read_username_from_file2() -> Result<String, io::Error> {
    // 对象本身返回类型就是Result; 且在Result类函数体内
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// ===============================================================
struct Info {
    name: String,
    age: i32,
    rating: i32,
}

// 不使用`?`
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
fn write_info2(info: &Info) -> io::Result<()> {
    let mut file = File::create("my_best_friends.txt")?;
    // Early return on error
    file.write_all(format!("name: {}\n", info.name).as_bytes())?;
    file.write_all(format!("age: {}\n", info.age).as_bytes())?;
    file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;
    Ok(())
}

// ==============================================================
fn test_io(s: &str) -> io::Result<()> {
    let mut f = File::create(s)?;
    f.write_all(b"Hello, world!")?;
    f.sync_data()?;
    Ok(())
}

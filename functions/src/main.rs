//
//http://rustdoc.saigao.fun/ch03-03-how-functions-work.html
fn main() {
    another_function(5);
    let _x = 6;
    let y = {
        let x = 3;
        //注意结尾没有分号的那一行，与大部分我们见过的代码行不同。
        //表达式并不包含结尾的分号。如果在表达式的结尾加上分号，
        //他就变成了语句，这也就使其不返回一个值。在接下来的探索中记住函数和表达式都返回值就行了
        x + 1
    };
    println!("The value of y is {}", y);
    println!("five = {}", five());

    let a = [1, 3, 5, 7];
    for element in a.iter() {
        println!("value : {}", element);
    }

    println!("==========================================");

    for element in (1..4).rev() {
        println!("value : {}", element);
    }
}

fn another_function(x: i32) {
    println!("x = {}", x);
}

fn five() -> i32 {
    5
}

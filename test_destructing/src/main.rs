#![allow(dead_code)]
//https://scotthuang_hzl.gitbooks.io/rust_articles/content/8destructuring_jie_gou.html

use En::*;
struct St {
    f1: i32,
    f2: f32
}

enum En {
    Var1,
    Var2,
    Var3(i32),
    Var4(i32, St, i32)
}

fn foo(x: &En) {
    match x {
        &Var1 => println!("first variant"),
        &Var3(5) => println!("third variant with number 5"),
        &Var3(x) => println!("third variant with number {} (not 5)", x),
        &Var4(3, St {f1: 3, f2: x}, 45) => {
            println!("destructuring an embedded struct, found {} in f2", x)
        }
        &Var4(_, ref x, _) => {
            println!("Some other Var4 with {} in f1 and {} in f2", x.f1, x.f2)
        }
        _ => println!("other (Var2)")
    }
}

fn main() {
    let st = St {f1: 2, f2: 3.3};
    let en = En::Var4(3, st, 45);
    foo(&en);

    let st = St {f1: 3, f2: 5.1};
    let en = En::Var4(3, st, 45);
    foo(&en)
}

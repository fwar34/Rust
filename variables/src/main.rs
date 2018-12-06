fn main() {
    let x = 5;
    //x = 6; cannot assign twice to immutable variable
    const _MAX_POINTS: u32 = 100_000;
    let x = x + 1;
    let x = x * 2;
    println!("x = {}", x);
    let tuple: (i32, f64, u8) = (500, 5.66, 98);
    println!("tuple = {:?}", tuple);
    let (x, _y, _z) = tuple;
    println!("x = {}", x);
    let a = tuple.0;
    println!("a = {}", a);
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("first = {}", first);
}

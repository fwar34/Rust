fn main() {
    let _v = vec![1, 2, 3];
    let _v: Vec<i32> = Vec::new();

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    //let last: Option<&i32> = v[100]; panic!
    let last: Option<&i32> = v.get(100);
    if last == None {
        println!("index error");
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let first = &v[0];
    //v.push(7);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }
}

fn main() {
    let mut num1 = 5;
    let mut f1 = |x: i32| num1 = num1 + x;
    let data1 = f1(2_i32);
    //let y = &mut num1; //error => there are two mutable borrow once

    let mut num4 = 5;
    {
        let mut f4 = |x| num4 += x;
        let data4 = f4(3);
        println!("data4: {:?}", data4);
    }
    let y2 = &mut num4;

    let num3 = 5;
    let f3 = || num3 + 3;
    let data3 = f3();
    let y = &num3;
    println!("num3: {:?} data3: {:?}", num3, data3);

    let mut num2 = 5;
    let mut f2 = move |x| {
        num2 += x;
        num2
    };
    let data2 = f2(3_i32);
    println!("num2: {:?} data2: {:?}", num2, data2);
}

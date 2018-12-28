//https://rustwiki.org/zh-CN//rust-by-example/flow_control/match/destructuring/destructure_pointers.html

fn main() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val)
    }

    match *reference {
        val => println!("Got a value via destructuring: {:?}", val)
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("Got a value via destructuring: {:?}", r)
    }

    match mut_value {
        ref mut r => {
            *r += 10;
            println!("We added 10. `mut_value`: {:?}", r);
        }
    }

    println!("mut_value: {:?}", mut_value);
}

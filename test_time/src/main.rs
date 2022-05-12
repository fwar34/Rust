use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut count = 0;
    loop {
        count += 1;
        if count == 1000000 {
            break;
        }
    }
    let duration = start.elapsed();
    println!("{:?}", duration);
}

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    thread::spawn(move || {
        let (ref lock, ref var) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        var.notify_one();
        println!("notify main thread");
    });

    let (ref lock, ref var) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("before wait");
        started = var.wait(started).unwrap();
        println!("after wait");
    }
}

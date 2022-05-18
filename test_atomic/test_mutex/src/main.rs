use std::{sync::{Arc, Mutex}, thread};

fn main() {
    let var = Arc::new(Mutex::new(5));
    let share_var = var.clone();

    let new_thread = thread::spawn(move || {
        let mut val = share_var.lock().unwrap();
        println!("share value in new thread: {}", *val);
        *val = 9;
    });

    new_thread.join().unwrap();
    println!("share value in main thread: {}", *(var.lock().unwrap()));
}

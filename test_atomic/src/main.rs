use std::{sync::{Arc, atomic::{AtomicUsize, Ordering}}, thread};

fn main() {
    let var: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(5));
    let share_var = var.clone();
    let new_thread = thread::spawn(move || {
        println!("share value in new thread: {}", share_var.load(Ordering::SeqCst));
        share_var.store(9, Ordering::SeqCst);
    });

    new_thread.join().unwrap();
    println!("share_var in main thread: {}", var.load(Ordering::SeqCst));
}

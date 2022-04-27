use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    // move 强制转移闭包里面使用的变量的所有权到闭包里面，比如此例中的 tx
    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("data"),
            String::from("data2"),
            String::from("data3"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // vals.into_iter().map(|val| {
        //     tx.send(val).unwrap();
        //     thread::sleep(Duration::from_secs(1));
        // })
    });

    for received in rx {
        println!("Got: {}", received);
    }

    handle.join().unwrap();
}

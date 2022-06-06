use std::sync::mpsc;
use std::thread::{spawn, sleep};

#[allow(dead_code)]
pub fn channels () {
    let (tx, rx) = mpsc::channel();

    spawn(move || {
        let mut times = 0;

        loop {
            let val = String::from("Hello, there...!");
            tx.send(val).unwrap();

            times += 1;

            sleep(std::time::Duration::from_millis(100));

            if times == 5 {
                println!("`tx` done all the tasks");
                break
            }
        }
    });

    loop {
        match rx.recv() {
            Ok (received) =>  println!("Got: {received}"),
            Err (e) => {
                println!("{}", e);
                break
            }
        }
    }
}

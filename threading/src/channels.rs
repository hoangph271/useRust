use std::sync::mpsc;
use std::thread::{spawn, sleep};
use std::time::Duration;

#[allow(dead_code)]
pub fn channels () {
    let (tx, rx) = mpsc::channel();

    let tx_cloned = tx.clone();
    spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx_cloned.send(val).unwrap();
            sleep(Duration::from_millis(265));
        }
    });

    spawn(move || {
        let mut times = 0;

        loop {
            tx.send(format!("What is UP #{times}...?")).unwrap();

            if times == 4 {
                break
            } else {
                times += 1;
                sleep(Duration::from_millis(265));
            }
        }
    });

    for received in rx {
        println!("{received}");
    }
}

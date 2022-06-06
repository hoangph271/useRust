use std::thread;
use std::time::Duration;

pub fn thread_spawn() {
    println!("thread_spawn() start");

    let thread_1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the thread #1", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let thread_2 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the thread #2", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();

    println!("thread_spawn() end");
}

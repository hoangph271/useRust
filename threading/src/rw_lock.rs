use std::sync::{Arc, RwLock};
use std::thread::{sleep, spawn};
use std::time::Duration;

pub fn rw_lock() {
    let mutex = RwLock::new(1);
    let arced_mutext = Arc::new(mutex);

    let thread_ids = (0..10).collect::<Vec<_>>();
    let threads = thread_ids.iter().map(|i| {
        let mutex_clone = arced_mutext.clone();
        let thread_id = i.clone();

        spawn(move || {
            // ? This is to prevent dead lock
            {
                let mut guard = mutex_clone.write().unwrap();
                *guard += 1;
                println!("{}", *guard);
            }
            // * This will work as well
            // drop(guard);

            if thread_id % 2 == 0 {
                sleep(Duration::from_millis(100));

                let mut guard = mutex_clone.write().unwrap();
                *guard += thread_id;
            }
        })
    });

    for thread in threads {
        thread.join().unwrap();
    }

    let final_result = arced_mutext.read().unwrap();
    println!("Finally: {final_result}");
}

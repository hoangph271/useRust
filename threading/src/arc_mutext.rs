use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

#[derive(Debug)]
struct User {
    id: usize,
    username: String,
}

pub fn arc_mutex() {
    let user = User {
        id: 1,
        username: "username".to_owned(),
    };
    let user_mutexed = Mutex::new(user);
    let user_arced = Arc::new(user_mutexed);

    let thread_1_user = user_arced.clone();
    let thread_1 = spawn(move || {
        let mut locked_user = thread_1_user.lock().unwrap();
        locked_user.username = "Username #1".to_owned();
        println!("Username in #1: {}", &locked_user.username);
    });

    let thread_2_user = user_arced.clone();
    let thread_2 = spawn(move || {
        sleep(Duration::from_millis(10));

        let mut locked_user = thread_2_user.lock().unwrap();

        println!("Username in #2: {}", &locked_user.username);

        locked_user.id = 2;
        locked_user.username = "New user in #2".to_owned();

        println!("Username in #2: {}", &locked_user.username);
        println!("User ID in #2: {}", &locked_user.id);
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();
}

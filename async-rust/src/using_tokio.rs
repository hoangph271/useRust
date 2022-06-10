use std::time::Duration;
use tokio::{
    spawn,
    time::sleep
};

pub async fn using_tokio() {
    let mut handlers = vec![];

    for i in 0..3 {
        let handler = spawn(async move {
            println!("@start: async_func({i})");

            let res = async_func(&i).await;

            println!("@end: {res}");
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.await.unwrap();
    }
}

async fn async_func (id: &u64) -> u64 {
    sleep(Duration::from_nanos(*id)).await;

    id * 2
}

pub async fn using_tokio() {
    let mut handlers = vec![];

    for i in 0..3 {
        let handler = tokio::spawn(async move {
            println!("@started: async_func({i})");
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.await.unwrap();
    }
}

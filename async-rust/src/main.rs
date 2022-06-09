mod using_tokio;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    using_tokio::using_tokio().await;
}

// fn main() {
//     let async_result = futures::executor::block_on(do_async());
//     println!("{}", async_result);
// }

#[allow(dead_code)]
async fn do_async() -> usize {
    println!("@started: async_func()");

    join_await().await;

    0
}

// #region // ? Without runtime
async fn async_task_0_0() {
    println!("async_task_0_0")
}
async fn async_task_0_1() {
    println!("async_task_0_1")
}
async fn async_task_1() {
    println!("async_task_1")
}
#[allow(dead_code)]
async fn join_await () -> usize {
    let task0 = async {
        async_task_0_0().await;
        async_task_0_1().await;
    };

    futures::join!(task0, async_task_1());

    0
}
// #endregion

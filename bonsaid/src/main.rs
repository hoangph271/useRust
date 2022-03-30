mod db;

// TODO:
// - Async runtime...?
// - C.R.U.D Posts

fn main() {
    futures::executor::block_on(db::can_create_posts()).unwrap();
}

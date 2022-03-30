mod db;

fn main() {
    futures::executor::block_on(db::can_create_posts()).unwrap();
}

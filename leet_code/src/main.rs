mod capitalize_title;

fn main() {
    println!(
        "{}",
        capitalize_title::Solution::capitalize_title("capiTalIze tHe titLe".to_owned())
    );
}

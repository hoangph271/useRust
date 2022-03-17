fn main() {
    println!("{}", Solution::to_lower_case("Hello, world!".to_owned()));
}

struct Solution {}
impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.chars()
            .map(|c| c.to_lowercase().to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

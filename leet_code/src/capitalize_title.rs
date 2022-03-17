pub struct Solution {}

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title
            .split(' ')
            .map(|word| match word.len() {
                0..=2 => word.to_lowercase(),
                _ => String::from_iter([&*word[..1].to_uppercase(), &word[1..].to_lowercase()]),
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

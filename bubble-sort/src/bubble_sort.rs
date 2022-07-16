pub fn bubble_sort(nums: &mut Vec<usize>) {
    for _ in 1..nums.len() {
        for j in 0..nums.len() - 1 {
            if nums[j] < nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}

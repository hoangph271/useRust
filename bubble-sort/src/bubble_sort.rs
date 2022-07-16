pub fn bubble_sort(nums: &mut Vec<usize>) {
    for _ in 1..nums.len() {
        for j in 0..nums.len() - 1 {
            if nums[j] < nums[j + 1] {
                let value_at_j = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = value_at_j;
            }
        }
    }
}

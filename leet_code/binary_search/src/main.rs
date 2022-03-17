fn main() {
    assert!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9) == 4);
    assert!(Solution::search(vec![2, 5], 2) == 0);
    assert!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2) == -1);
    assert!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 12) == 5);
}

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // ? Just a linear search
        // for (i, num) in nums.iter().enumerate() {
        //     if *num == target {
        //         return i.try_into().unwrap();
        //     }
        // }

        if nums.is_empty() {
            return -1;
        }

        let pivot = ((nums.len() as f32) / 2.0).floor() as usize;

        if nums[pivot] == target {
            return pivot as i32;
        }

        let res = if nums[pivot] < target {
            Solution::search(nums[pivot + 1..].to_vec(), target)
        } else {
            Solution::search(nums[..pivot].to_vec(), target)
        };

        if res < 0 {
            return -1;
        }

        if nums[pivot] < target {
            (pivot as i32) + res + 1
        } else {
            res
        }
    }
}

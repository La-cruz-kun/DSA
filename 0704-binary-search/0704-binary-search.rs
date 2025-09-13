impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = (nums.len() - 1) as i32;
        for _ in 0..nums.len() {
            let middle = (start + end) / 2;
            if nums[middle as usize] == target {
                return middle;
            } else if nums[middle as usize] > target {
                end = middle;
            } else {
                start = middle + 1;
            }
        }

        return -1;
    }
}

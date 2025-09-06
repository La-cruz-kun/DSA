impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prefix = vec![1; n];
        let mut suffix = vec![1; n];
        for i in 1..n {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }
        for i in (0..n - 1).rev() {
            suffix[i] = suffix[i + 1] * nums[i + 1];
        }
        let result: Vec<_> = suffix
            .iter()
            .zip(prefix.iter())
            .map(|(x, y)| x * y)
            .collect();
        result
    }
}

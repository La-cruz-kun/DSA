use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let map = nums.into_iter().fold(HashMap::new(), |mut acc, i| {
            acc.entry(i).and_modify(|x| *x += 1).or_insert(1);
            acc
        });

        let mut sort_map: Vec<_> = map.into_iter().collect();
        sort_map.sort_by(|x, y| y.1.cmp(&x.1));
        sort_map.truncate(k as usize);
        let (key, _): (Vec<i32>, Vec<i32>) = sort_map.into_iter().unzip();

        key
    }
}

use std::collections::HashMap;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums_hash: HashMap<i32, bool> = HashMap::new();
        for i in nums {
            if nums_hash.contains_key(&i){
                return true;
            }
            nums_hash.insert(i, true);
        }
        false
    }
}

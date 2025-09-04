use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut output: Vec<Vec<String>> = Vec::new();
        let mut temp: HashMap<Vec<char>, Vec<usize>> = HashMap::new();
        for (i, val) in strs.iter().enumerate() {
            let mut n_val = val.clone().chars().collect::<Vec<_>>();
            n_val.sort();
            temp.entry(n_val)
                .and_modify(|e| e.push(i))
                .or_insert(vec![i]);
        }
        for (_, val) in temp {
            let mut values = Vec::new();
            for i in val {
                values.push(strs[i].clone());
            }
            output.push(values);
        }
        output
    }
}

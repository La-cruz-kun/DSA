impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s: Vec<char> = s.chars().into_iter().collect();
        let open = vec!['(', '{', '['];
        let close = vec![')', '}', ']'];
        let mut val = Vec::new();
        for i in 0..s.len() {
            if open.contains(&s[i]) {
                val.push(s[i]);
            } else if let Some(index) = close.iter().position(|&x| x == s[i]) {
                if val.len() == 0 {
                    return false;
                }
                if val.pop().unwrap() == open[index] {
                    continue;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        if val.len() == 0 {
            return true;
        }
        false
    }
}

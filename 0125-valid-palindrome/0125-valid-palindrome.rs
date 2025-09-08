impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let t: String = s.to_lowercase().matches(char::is_alphanumeric).collect();
        let n = t.len();
        let t: Vec<_> = t.chars().collect();
        for i in 0..n {
            if t[i] != t[n - 1 - i] {
                return false;
            }
        }
        true
    }
}

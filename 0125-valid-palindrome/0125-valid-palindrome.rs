impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let t: String = s.to_lowercase().matches(char::is_alphanumeric).collect();
        let n = t.len();
        let t: Vec<_> = t.chars().collect();
        (0..n / 2).all(|x| t[x] == t[n - 1 - x])
    }
}

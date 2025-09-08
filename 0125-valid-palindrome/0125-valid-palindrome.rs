impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let v = s.to_lowercase();
        let t: Vec<_> = v
            .matches(char::is_alphanumeric)
            .map(|x| x.chars().next().unwrap())
            .collect();
        let n = t.len();
        (0..n / 2).all(|x| t[x] == t[n - 1 - x])
    }
}

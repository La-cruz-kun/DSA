impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut t_vec = t.chars().collect::<Vec<_>>();
        let mut s_vec = s.chars().collect::<Vec<_>>();
        t_vec.sort();
        s_vec.sort();
        let longest = t_vec.len().max(s_vec.len());
        for (i, j) in t_vec.into_iter().map(Some).chain(std::iter::repeat(None)).zip(s_vec.into_iter().map(Some).chain(std::iter::repeat(None))).take(longest){
            if i != j{
                return false;
            }
        }
        true

    }
}

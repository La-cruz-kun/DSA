impl Solution {
    pub fn is_valid(s: String) -> bool {
        let open = ['(', '{', '['];
        let close = [')', '}', ']'];

        s.chars().try_fold(Vec::new(), |mut stack, c| {
            if let Some(idx) = open.iter().position(|&o| o == c) {
                stack.push(open[idx]);
                Some(stack)
            } else if let Some(idx) = close.iter().position(|&cl| cl == c) {
                match stack.pop() {
                    Some(top) if top == open[idx] => Some(stack),
                    _ => None, // invalid closing
                }
            } else {
                None // invalid char
            }
        })
        .map(|stack| stack.is_empty())
        .unwrap_or(false)
    }
}

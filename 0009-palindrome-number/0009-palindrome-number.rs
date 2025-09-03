impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut new_x = x.to_string();
        let mut y = String::new();
        for i in 0..new_x.len() {
            y.push(new_x.pop().unwrap());
        }
        println!("x: {x}");
        println!("y: {y}");
        if y == x.to_string() {
            return true;
        }
        return false;
    }
}
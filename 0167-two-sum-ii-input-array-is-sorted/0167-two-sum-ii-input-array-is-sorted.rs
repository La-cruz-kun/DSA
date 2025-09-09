impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut running = true;
        let mut first = 0;
        let mut last = numbers.len() - 1;
        while running {
            let compliment = target - numbers[last];
            if numbers[first] > compliment {
                last -= 1;
            } else if numbers[first] < compliment {
                first += 1;
            } else if numbers[first] == compliment {
                running = false;
            } else if numbers[last] > target {
                last -= 1;
            }
        }
        vec![first as i32 + 1, last as i32 + 1]
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
    let mut previous = ' ';
    let mut result = 0;
    for i in s.chars() {
        if i == 'I' {
            result += 1;

            previous = i;
        } else if i == 'V' {
            result += 5;
            if previous == 'I' {
                result -= 2;
            }
            previous = i;
        } else if i == 'L' {
            result += 50;
            if previous == 'X' {
                result -= 20;
            }
            previous = i;
        } else if i == 'X' {
            result += 10;
            if previous == 'I' {
                result -= 2;
            }
            previous = i;
        } else if i == 'C' {
            result += 100;
            if previous == 'X' {
                result -= 20;
            }
            previous = i;
        } else if i == 'D' {
            result += 500;
            if previous == 'C' {
                result -= 200;
            }
            previous = i;
        } else if i == 'M' {
            result += 1000;
            if previous == 'C' {
                result -= 200;
            }
            previous = i;
        }
    }
    result
}

}
use std::collections::HashMap;

pub fn is_duplicate(n: &Vec<char>) -> bool {
    let mut map = HashMap::new();
    for i in n {
        if *i == '.' {
            continue;
        }
        if *map.entry(i).and_modify(|x| *x += 1).or_insert(1) > 1 {
            return false;
        }
    }
    true
}
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut map: HashMap<usize, Vec<char>> = HashMap::new();
        let mut map_box: HashMap<usize, Vec<char>> = HashMap::new();
        let n = board.len();
        for (i, val) in board.iter().enumerate() {
            if is_duplicate(&val) == false {
                return false;
            }
            for j in 0..n {
                map.entry(j)
                    .and_modify(|x| x.push(val[j]))
                    .or_insert(vec![val[j]]);
            }
            for k in 0..n {
                let mut count = 0;
                if k <= 2 {
                } else if k >= 3 && k <= 5 {
                    count += 1;
                } else if k >= 6 && k <= 8 {
                    count += 2;
                }
                if i <= 2 {
                } else if i >= 3 && i <= 5 {
                    count += 3;
                } else if i >= 6 && i <= 8 {
                    count += 6;
                }
                map_box
                    .entry(count)
                    .and_modify(|x| x.push(val[k]))
                    .or_insert(vec![val[k]]);
            }
        }
        for i in 0..n {
            if is_duplicate(map.entry(i).or_default()) == false {
                return false;
            }
            if is_duplicate(map_box.entry(i).or_default()) == false {
                return false;
            }
        }
        true
    }
}

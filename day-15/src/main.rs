use std::collections::HashMap;

const INPUT: &str = "0,8,15,2,12,1,4";

fn say_number(numbers: &[usize], count: usize) -> usize {
    let mut used_numbers: HashMap<usize, Vec<usize>> = HashMap::new();
    for (idx, num) in numbers.iter().enumerate() {
        used_numbers.insert(*num, vec![idx + 1]);
    }
    let mut last_number = *numbers.last().unwrap();
    for turn in numbers.len()..count {
        if let Some(appearances) = used_numbers.get(&last_number) {
            if appearances.len() < 2 {
                last_number = 0;
                if let Some(a) = used_numbers.get_mut(&last_number) {
                    a.push(turn + 1);
                } else {
                    used_numbers.insert(last_number, vec![turn + 1]);
                }
            } else {
                let v: Vec<_> = appearances.iter().cloned().rev().take(2).collect();
                last_number = v[0] - v[1];
                if let Some(a) = used_numbers.get_mut(&last_number) {
                    a.push(turn + 1);
                } else {
                    used_numbers.insert(last_number, vec![turn + 1]);
                }
            }
        } else {
            last_number = 0;
            used_numbers.insert(last_number, vec![turn + 1]);
        }
    }

    last_number
}

fn main() {
    let numbers: Vec<usize> = INPUT.trim().split(",").map(|s| s.parse().unwrap()).collect();
    println!("Part 1: {}", say_number(&numbers, 2020));
    println!("Part 2: {}", say_number(&numbers, 30000000));
}

#[cfg(test)]
mod test {
    use super::say_number;

    #[test]
    fn test_it() {
        assert_eq!(say_number(&[1, 3, 2], 2020), 1);
        assert_eq!(say_number(&[2, 1, 3], 2020), 10);
        assert_eq!(say_number(&[1, 2, 3], 2020), 27);
        assert_eq!(say_number(&[2, 3, 1], 2020), 78);
        assert_eq!(say_number(&[3, 2, 1], 2020), 438);
        assert_eq!(say_number(&[3, 1, 2], 2020), 1836);
    }
}

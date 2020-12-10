use std::collections::HashSet;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input");
const LOOKUP_SIZE: usize = 25;

fn find_first_invalid(numbers: &[i64]) -> i64 {
    let mut queue = VecDeque::new();
    let mut lookup = HashSet::new();

    for num in numbers.iter().cloned() {
        if queue.len() < LOOKUP_SIZE {
            queue.push_back(num);
            lookup.insert(num);
        } else {
            if !queue.iter().any(|v| lookup.contains(&(num - v))) {
                return num;
            }
            let first = queue.pop_front().unwrap();
            lookup.remove(&first);
            queue.push_back(num);
            lookup.insert(num);
        }
    }
    unreachable!();
}

fn sum_from(s: usize, numbers: &[i64], target: i64) -> Option<usize> {
    let mut sum = 0;
    for end in s..numbers.len() {
        sum += numbers[end];
        if sum == target {
            return Some(end + 1);
        }
        if sum > target {
            return None;
        }
    }
    None
}

fn find_sum(numbers: &[i64], target: i64) -> (usize, usize) {
    for start in 0..numbers.len() - 1 {
        if let Some(idx) = sum_from(start, numbers, target) {
            return (start, idx);
        }
    }
    unreachable!();
}

fn main() {
    let numbers: Vec<i64> = INPUT.trim().lines().map(|l| l.parse().unwrap()).collect();
    let num = find_first_invalid(&numbers);
    println!("Number is {}", num);
    let (s, e) = find_sum(&numbers, num);
    println!("Solution: {}", numbers[s..e].iter().min().unwrap() + numbers[s..e].iter().max().unwrap());
}

use std::collections::HashSet;

const INPUT: &str = include_str!("../input");

fn count_answers(s: &str) -> usize {
    let mut set = HashSet::new();
    for line in s.lines() {
        for c in line.chars() {
            set.insert(c);
        }
    }
    set.len()
}

fn count_answers_part2(s: &str) -> usize {
    let all_answers: Vec<HashSet<char>> = s.lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut it = all_answers.iter();
    if let Some(first) = it.next() {
        all_answers.iter().fold(first.clone(), |acc, x| acc.intersection(x).cloned().collect()).len()
    } else {
        0
    }
}

fn main() {
    let x: usize = INPUT.split("\n\n")
        .map(count_answers)
        .sum();
    println!("Part 1: {}", x);
    let x: usize = INPUT.split("\n\n")
        .map(count_answers_part2)
        .sum();
    println!("Part 2: {}", x);
}

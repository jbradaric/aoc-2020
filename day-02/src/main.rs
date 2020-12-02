const INPUT: &str = include_str!("../input");

#[derive(Debug)]
struct Policy {
    pub min: usize,
    pub max: usize,
    pub letter: char,
}

fn parse_policy(text: &str) -> Policy {
    let tmp: Vec<_> = text.split(' ').collect();
    let letter = tmp[1].chars().next().unwrap();

    let tmp: Vec<_> = tmp[0].split('-').collect();

    Policy {
        min: tmp[0].parse().unwrap(),
        max: tmp[1].parse().unwrap(),
        letter,
    }
}

fn parse_line(line: &str) -> (Policy, &str) {
    let tmp: Vec<_> = line.split(':').collect();
    (parse_policy(tmp[0]), tmp[1].trim_start())
}

fn check_password(line: &&str) -> bool {
    let (policy, password) = parse_line(*line);
    let count = password.chars().filter(|c| *c == policy.letter).count();
    count >= policy.min && count <= policy.max
}

fn check_password2(line: &&str) -> bool {
    let (policy, password) = parse_line(*line);
    let c1 = password.chars().nth(policy.min - 1).unwrap_or('-');
    let c2 = password.chars().nth(policy.max - 1).unwrap_or('-');
    (c1 == policy.letter || c2 == policy.letter) && c1 != c2
}

fn main() {
    let num_valid = INPUT.lines().filter(check_password).count();
    println!("Part 1: num valid passwords: {}", num_valid);

    let num_valid = INPUT.lines().filter(check_password2).count();
    println!("Part 2: num valid passwords: {}", num_valid);
}

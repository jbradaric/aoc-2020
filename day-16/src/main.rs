use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = include_str!("../input");

#[derive(Debug)]
struct Range {
    pub min: u32,
    pub max: u32,
}

impl Range {
    fn validate(&self, n: u32) -> bool {
        self.min <= n && n <= self.max
    }
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("-").map(|s| s.trim().parse().unwrap()).collect();
        Ok(Range {
            min: parts[0],
            max: parts[1],
        })
    }
}

#[derive(Debug)]
struct Condition {
    pub name: String,
    pub ranges: Vec<Range>,
}

impl Condition {
    fn validate(&self, n: u32) -> bool {
        self.ranges.iter().any(|r| r.validate(n))
    }
}

#[derive(Debug)]
struct ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError {}
    }
}

impl FromStr for Condition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(":").collect();
        Ok(Condition {
            name: parts[0].trim().to_string(),
            ranges: parts[1]
                .split("or")
                .map(|s| s.trim().parse().unwrap())
                .collect(),
        })
    }
}

struct Ticket {
    pub numbers: Vec<u32>,
}

fn main() {
    let parts: Vec<_> = INPUT.trim().split("\n\n").collect();
    let conditions: Vec<Condition> = parts[0].lines().map(|l| l.parse().unwrap()).collect();
    let my_ticket = Ticket {
        numbers: parts[1]
            .split("\n")
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect(),
    };

    let s: u32 = parts[2]
        .lines()
        .skip(1)
        .map(|l| {
            l.split(",")
                .map(|s| s.parse().unwrap())
                .filter(|n| conditions.iter().all(|c| !c.validate(*n)))
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("Ticket scanning error rate: {}", s);

    let valid_tickets: Vec<Ticket> = parts[2]
        .lines()
        .skip(1)
        .map(|l| {
            l.split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|v| v.iter().all(|&x| conditions.iter().any(|c| c.validate(x))))
        .map(|v| Ticket { numbers: v })
        .collect();

    let mut candidates: Vec<Vec<&Condition>> = vec![];
    for field_idx in 0..valid_tickets[0].numbers.len() {
        let mut valid_conds = vec![];
        for cond in conditions.iter() {
            if valid_tickets
                .iter()
                .all(|t| cond.validate(t.numbers[field_idx]))
            {
                valid_conds.push(cond);
            }
        }
        candidates.push(valid_conds);
    }

    let mut resolved_conds: HashMap<usize, String> = HashMap::new();
    let mut resolved_names: HashSet<String> = HashSet::new();
    for (field_idx, conds) in candidates.iter().enumerate() {
        if conds.len() == 1 {
            resolved_conds.insert(field_idx, conds[0].name.clone());
            resolved_names.insert(conds[0].name.clone());
        }
    }
    while resolved_conds.len() < candidates.len() {
        for (field_idx, conds) in candidates.iter().enumerate() {
            if resolved_conds.contains_key(&field_idx) {
                continue;
            }
            let valid: Vec<_> = conds.iter().filter(|c| !resolved_names.contains(&c.name)).collect();
            if valid.len() == 1 {
                resolved_conds.insert(field_idx, valid[0].name.clone());
                resolved_names.insert(valid[0].name.clone());
            }
        }
    }
    let mut result: u64 = 1;
    for (field_idx, value) in my_ticket.numbers.iter().enumerate() {
        if resolved_conds[&field_idx].starts_with("departure") {
            result *= *value as u64;
        }
    }
    println!("Part 2 result: {}", result);
}

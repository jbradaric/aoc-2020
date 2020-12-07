use regex::Regex;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input");

fn count_bags(color: &String, m: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>) {
    if !visited.contains(color) {
        visited.insert(color.clone());
        m.get(color)
            .map(|c| c.iter().map(|c| count_bags(c, m, visited)).for_each(|_| {}));
    }
}

fn count_bags_in(color: &String, m: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    return 1 + m
        .get(color)
        .map(|v| v.iter().map(|(cnt, clr)| cnt * count_bags_in(clr, m)).sum())
        .unwrap_or(0);
}

fn main() {
    let start = Regex::new(r"(?x)(?P<container>\w+\s\w+)\sbags\scontain\s").unwrap();
    let rest = Regex::new(r"(?x)(?P<count>\d+)\s(?P<rest>\w+\s\w+)\sbags?(,\s|\.)").unwrap();
    let mut map: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    for line in INPUT.trim().lines() {
        let container = start.captures(line).unwrap()["container"].to_string();
        let mut contents = Vec::new();
        for rest in rest.captures_iter(line) {
            let count = rest["count"].parse().unwrap();
            contents.push((count, rest["rest"].to_string()));
        }
        map.insert(container, contents);
    }

    let mut rev_map: HashMap<String, Vec<String>> = HashMap::new();
    for spec in map.iter() {
        for bag in spec.1 {
            if !rev_map.contains_key(&bag.1) {
                rev_map.insert(bag.1.clone(), Vec::new());
            }
            rev_map.get_mut(&bag.1).unwrap().push(spec.0.clone());
        }
    }
    let mut visited = HashSet::new();
    count_bags(&"shiny gold".to_string(), &rev_map, &mut visited);
    println!("Visited: {}", visited.len() - 1); // don't count "shiny gold"

    println!(
        "Total contents: {}",
        count_bags_in(&"shiny gold".to_string(), &map) - 1
    );
}

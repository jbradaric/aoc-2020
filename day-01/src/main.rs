use std::collections::HashSet;

const INPUT: &str = include_str!("../input");

fn find_sum(set: &HashSet<i32>, total: i32) -> Option<(i32, i32)> {
    for entry in set.iter() {
        if set.contains(&(total - entry)) {
            return Some((*entry, total - entry));
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut set: HashSet<i32> = HashSet::new();
    for line in INPUT.lines() {
        set.insert(line.parse()?);
    }
    println!("Part 1\n======");
    if let Some((x, y)) = find_sum(&set, 2020) {
        println!("Found {} and {}: {}\n", x, y, x * y);
    }

    println!("Part 2\n======");
    for entry in set.iter() {
        if let Some((x, y)) = find_sum(&set, 2020 - entry) {
            println!("Found {}, {} and {}: {}", entry, x, y, entry * x * y);
            break;
        }
    }
    Ok(())
}

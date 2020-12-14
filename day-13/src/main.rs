const INPUT: &str = include_str!("../input");

fn main() {
    let mut lines = INPUT.trim().lines();
    let estimate: u32 = lines.next().unwrap().parse().unwrap();
    let departures: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|s| s != &"x")
        .map(|s| s.parse().unwrap())
        .collect();
    let r = departures
        .iter()
        .map(|x| (estimate / x + 1) * x - estimate)
        .enumerate()
        .min_by_key(|(_, x)| x.clone())
        .unwrap();
    println!("Part 1: {}", departures[r.0] * r.1);

    // Part 2: Chinese remainder theorem and Wolfram Alpha :)
}

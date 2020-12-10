use std::collections::VecDeque;

const INPUT: &str = include_str!("../input");


fn count_differences(adapters: &[i32]) -> (i32, i32) {
    let mut diff_1 = 0;
    let mut diff_3 = 0;

    for (a1, a2) in adapters.iter().zip(adapters.iter().skip(1)) {
        if a2 - a1 == 1 {
            diff_1 += 1;
        } else if a2 - a1 == 3 {
            diff_3 += 1;
        }
    }

    (diff_1, diff_3)
}

fn get_adapters(s: &str) -> Vec<i32> {
    let mut adapters: Vec<i32> = s.trim().lines().map(|l| l.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    adapters
}

type Path = Vec<i32>;
type Queue = VecDeque<Path>;

fn count_arrangements(seq: &[i32]) -> i64 {
    if seq.len() <= 2 {
        return 1;
    }
    let mut todo: Queue = VecDeque::new();
    todo.push_back(vec![seq[0]]);
    let dest = seq.last().unwrap();
    let mut count = 0;
    while !todo.is_empty() {
        let path = todo.pop_front().unwrap();
        let lastnode = path.last().unwrap();
        if lastnode == dest {
            count += 1;
        }
        let idx = seq.iter().enumerate().find(|i| i.1 == lastnode).unwrap().0;
        for x in idx + 1..seq.len() {
            if seq[x] - lastnode > 3 {
                break;
            }
            let mut path = path.clone();
            path.push(seq[x]);
            todo.push_back(path);
        }
    }

    count
}

fn count_paths(adapters: &[i32]) -> i64 {
    let mut sequences: Vec<&[i32]> = vec![];

    let mut start = 0;
    while start < adapters.len() - 1 {
        for end in start + 1 .. adapters.len() {
            if adapters[end] - adapters[end - 1] >= 3 {
                sequences.push(&adapters[start..end]);
                start = end;
                break;
            }
        }
    }

    sequences.iter().map(|s| count_arrangements(s)).product()
}

fn main() {
    let adapters = get_adapters(INPUT);
    let (d1, d2) = count_differences(&adapters);
    println!("Part 1 solution: {}", d1 * d2);

    println!("Part 2 solution: {}", count_paths(&adapters));
}

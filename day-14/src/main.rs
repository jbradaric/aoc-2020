use std::collections::HashMap;

const INPUT: &str = include_str!("../input");

fn mask_str_to_bits(s: &str) -> (u64, u64) {
    let mut zeros = 1;
    let mut ones = 0;
    for ch in s.chars() {
        match ch {
            '0' => {
                zeros &= !(1u64);
            }
            '1' => {
                zeros |= 1;
                ones |= 1;
            }
            'X' => {
                zeros |= 1;
                ones |= 0;
            }
            _ => unreachable!(),
        };
        zeros <<= 1;
        ones <<= 1;
    }

    // Shift right because mask were shifted left after the last mask char
    (zeros >> 1, ones >> 1)
}

fn mask_address(start_mask: u64, float_bits: &[usize]) -> Vec<u64> {
    if float_bits.len() == 0 {
        return vec![start_mask];
    }

    let mut todo = vec![];
    let mut it = float_bits.iter();
    let idx = it
        .next()
        .expect("float_bits not empty, but iter.next() returned None");
    todo.push(vec![(idx, 0)]);
    todo.push(vec![(idx, 1)]);

    for next_bit in it {
        let mut tmp = vec![];
        for path in todo {
            let mut p = path.clone();
            p.push((next_bit, 0));
            tmp.push(p);
            let mut p = path.clone();
            p.push((next_bit, 1));
            tmp.push(p);
        }
        todo = tmp;
    }

    let mut masks = vec![];
    for path in todo {
        let mut mask = start_mask;
        for (idx, value) in path {
            if value == 1 {
                mask |= 1 << idx;
            } else {
                mask &= !(1 << idx);
            }
        }
        masks.push(mask);
    }
    masks
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask = "";
    let (mut zeros, mut ones) = mask_str_to_bits(mask);
    for line in INPUT.trim().lines() {
        if line.starts_with("mask =") {
            mask = line.trim_start_matches("mask =").trim();
            let r = mask_str_to_bits(mask);
            zeros = r.0;
            ones = r.1;
        } else {
            let parts: Vec<_> = line.split("=").map(|s| s.trim()).collect();
            let addr: usize = parts[0]
                .trim_start_matches("mem[")
                .trim_end_matches("]")
                .parse()?;
            let value: u64 = parts[1].parse()?;
            let res = (value & zeros) | ones;
            memory.insert(addr, res);
        }
    }

    let sum: u64 = memory.values().filter(|k| **k != 0).sum();
    println!("Part 1: {}", sum);

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = "";
    for line in INPUT.trim().lines() {
        if line.starts_with("mask =") {
            mask = line.trim_start_matches("mask =").trim();
        } else {
            let parts: Vec<_> = line.split("=").map(|s| s.trim()).collect();
            let mut addr = parts[0]
                .trim_start_matches("mem[")
                .trim_end_matches("]")
                .parse()?;
            let value: u64 = parts[1].parse()?;
            let float_bits: Vec<_> = mask
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == 'X')
                .map(|(idx, _)| 35 - idx)
                .collect();
            for (idx, v) in mask.chars().enumerate() {
                if v == '1' {
                    addr |= 1 << (mask.len() - idx - 1);
                }
            }
            for m in mask_address(addr, &float_bits) {
                memory.insert(m, value);
            }
        }
    }

    let sum: u64 = memory.values().filter(|k| **k != 0).sum();
    println!("Part 2: {}", sum);

    Ok(())
}

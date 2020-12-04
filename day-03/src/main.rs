const INPUT: &str = include_str!("../input");

fn follow_path(map: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let mut num_trees = 0;
    let mut pos_left = 0;
    let mut pos_top = 0;
    loop {
        if map[pos_top][pos_left % map[pos_top].len()] == '#' {
            num_trees += 1;
        }
        pos_left += right;
        pos_top += down;
        if pos_top >= map.len() {
            break;
        }
    }

    num_trees
}

fn main() {
    // parse map
    let mut map: Vec<Vec<_>> = Vec::new();
    for line in INPUT.lines() {
        let line_vec = line.chars().collect();
        map.push(line_vec);
    }

    println!("Trees encountered: {}", follow_path(&map, 3, 1));

    let mut prod = 1;
    for slope in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        prod *= follow_path(&map, slope.0, slope.1);
    }

    println!("Product of trees encountered on all slopes: {}", prod);
}

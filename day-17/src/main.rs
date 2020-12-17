use std::collections::HashSet;
use std::hash::Hash;
const INPUT: &str = include_str!("../input");

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point3D(i32, i32, i32);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point4D(i32, i32, i32, i32);

trait Point {
    type Item;
    fn neighbors(&self) -> Vec<Self::Item>;
}

impl Point for Point3D {
    type Item = Point3D;

    fn neighbors(&self) -> Vec<Self::Item> {
        let mut res = vec![];
        for x_off in -1..=1 {
            for y_off in -1..=1 {
                for z_off in -1..=1 {
                    let n = Point3D(self.0 + x_off, self.1 + y_off, self.2 + z_off);
                    if &n != self {
                        res.push(n);
                    }
                }
            }
        }
        res
    }
}

impl Point for Point4D {
    type Item = Point4D;

    fn neighbors(&self) -> Vec<Self::Item> {
        let mut res = vec![];
        for x_off in -1..=1 {
            for y_off in -1..=1 {
                for z_off in -1..=1 {
                    for w_off in -1..=1 {
                        let n = Point4D(
                            self.0 + x_off,
                            self.1 + y_off,
                            self.2 + z_off,
                            self.3 + w_off,
                        );
                        if &n != self {
                            res.push(n);
                        }
                    }
                }
            }
        }
        res
    }
}

#[derive(Clone)]
struct Space<T: Point<Item = T> + Eq + Hash + Copy> {
    pub active_points: HashSet<T>,
}

impl<T: Point<Item = T> + Eq + Hash + Copy> Space<T> {
    fn new() -> Self {
        Self {
            active_points: HashSet::new(),
        }
    }

    fn get(&self, p: &T) -> char {
        if self.active_points.contains(p) {
            '#'
        } else {
            '.'
        }
    }

    fn activate(&mut self, p: T) {
        self.active_points.insert(p);
    }

    fn deactivate(&mut self, p: &T) {
        self.active_points.remove(p);
    }

    fn active_neighbors(&self, p: &T) -> usize {
        let mut count = 0;

        for n in p.neighbors() {
            if self.get(&n) == '#' {
                count += 1;
            }
        }

        count
    }

    fn next(&self) -> Self {
        let mut result = self.clone();

        // Deactivate active cubes
        for p in &self.active_points {
            let active_neighbors = self.active_neighbors(p);
            if active_neighbors != 2 && active_neighbors != 3 {
                result.deactivate(p);
            }
        }

        // Activate inactive cubes
        let mut seen = HashSet::new();
        for p in &self.active_points {
            for n in p.neighbors() {
                if seen.contains(&n) {
                    continue;
                }
                seen.insert(n);
                let active_neighbors = self.active_neighbors(&n);
                if active_neighbors == 3 {
                    result.activate(n);
                }
            }
        }

        result
    }

    fn num_active(&self) -> usize {
        self.active_points.len()
    }
}

fn main() {
    let mut space = Space::new();
    for (row_idx, line) in INPUT.trim().lines().enumerate().map(|(i, l)| (i as i32, l)) {
        for (col_idx, char) in line.chars().enumerate().map(|(i, c)| (i as i32, c)) {
            if char == '#' {
                space.activate(Point3D(row_idx, col_idx, 0));
            }
        }
    }

    for _ in 0..6 {
        space = space.next();
    }
    println!("Part 1: {}", space.num_active());

    let mut space = Space::new();
    for (row_idx, line) in INPUT.trim().lines().enumerate().map(|(i, l)| (i as i32, l)) {
        for (col_idx, char) in line.chars().enumerate().map(|(i, c)| (i as i32, c)) {
            if char == '#' {
                space.activate(Point4D(row_idx, col_idx, 0, 0));
            }
        }
    }

    for _ in 0..6 {
        space = space.next();
    }
    println!("Part 2: {}", space.num_active());
}

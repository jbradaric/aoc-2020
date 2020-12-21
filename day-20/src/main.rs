use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = include_str!("../input");

#[derive(Debug)]
struct ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError {}
    }
}

#[derive(Default, Clone)]
struct Tile {
    pub id: u64,
    pub pixels: Vec<Vec<char>>
}

impl Tile {
    fn rotate(&self) -> Self {
        let mut new_pixels = self.pixels.clone();
        for (idx, row) in self.pixels.iter().enumerate() {
            for c in 0..10 {
                new_pixels[c][10 - idx - 1] = row[c];
            }
        }
        Tile { id: self.id, pixels: new_pixels }
    }

    fn flip(&self) -> Self {
        let mut pixels = self.pixels.clone();
        pixels.reverse();
        Tile { id: self.id, pixels }
    }

    fn borders(&self) -> Vec<Vec<char>> {
        let mut result = vec![];
        result.push(self.pixels[0].clone());
        result.push((0..self.pixels.len()).map(|i| self.pixels[i][9]).collect());
        result.push(self.pixels.last().unwrap().clone());
        result.push((0..self.pixels.len()).map(|i| self.pixels[i][0]).collect());

        result
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        for row in &self.pixels {
            for px in row {
                write!(f, "{}", px)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for Tile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.trim().lines().next().unwrap();
        let id: u64 = line
            .strip_prefix("Tile ")
            .and_then(|s| s.strip_suffix(":"))
            .and_then(|s| s.parse().ok())
            .unwrap();
        let pixels = s.trim()
            .lines()
            .skip(1)
            .map(|l| l.chars().collect())
            .collect();
        Ok(Tile { id, pixels })
    }
}

fn matching_borders(t: &Tile, other: &Tile) -> bool {
    let mut t_copy = t.clone();
    for _ in 0..4 {
        let other_borders = other.borders();
        for b in t_copy.borders() {
            if other_borders.contains(&b) {
                return true;
            }
        }
        t_copy = t_copy.rotate();
    }
    return false;
}

fn is_corner(tile: &Tile, others: &[Tile]) -> bool {
    let mut matches = HashSet::new();
    for t in others {
        if tile.id == t.id {
            continue;
        }
        if matching_borders(tile, t) {
            matches.insert(t.id);
        }
    }
    matches.len() == 2
}

fn main() {
    let tiles: Vec<Tile> = INPUT.trim().split("\n\n").map(|s| s.parse().unwrap()).collect();
    let corners = tiles.iter().filter(|t| is_corner(t, &tiles))
        .map(|t| t.id).fold(1, |a, b| a * b);
    println!("Product: {}", corners);
}

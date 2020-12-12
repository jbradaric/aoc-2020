const INPUT: &str = include_str!("../input");

#[derive(Clone, PartialEq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Position::Floor => ".",
            Position::Empty => "L",
            Position::Occupied => "#"
        })
    }
}

impl Position {
    fn from(c: char) -> Self {
        match c {
            '.' => Position::Floor,
            'L' => Position::Empty,
            '#' => Position::Occupied,
            _ => unreachable!(),
        }
    }
}

fn count_adjacent(seats: &Vec<Vec<Position>>, row: usize, col: usize) -> usize {
    let row = row as i64;
    let col = col as i64;
    let mut count = 0;
    let num_cols = seats[0].len() as i64;
    for i in row - 1..=row + 1 {
        if i < 0 || i >= seats.len() as i64 {
            continue;
        }
        for j in col - 1..=col + 1 {
            if i == row && j == col {
                continue;
            }
            if j < 0 || j >= num_cols {
                continue;
            }
            if seats[i as usize][j as usize] == Position::Occupied {
                count += 1;
            }
        }
    }
    count
}

fn count_visible(seats: &Vec<Vec<Position>>, row: usize, col: usize) -> i64 {
    let row = row as i64;
    let col = col as i64;
    let mut visible = 0;
    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            if row_offset == 0 && col_offset == 0 {
                continue;
            }
            let mut r = row;
            let mut c = col;
            loop {
                if r == row && c == col {
                    r += row_offset;
                    c += col_offset;
                    continue;
                }
                if r < 0 || r >= seats.len() as i64 {
                    break;
                }
                if c < 0 || c >= seats[0].len() as i64 {
                    break;
                }
                if seats[r as usize][c as usize] == Position::Floor {
                    r += row_offset;
                    c += col_offset;
                    continue;
                } else {
                    if seats[r as usize][c as usize] == Position::Occupied {
                        visible += 1;
                    }
                    break;
                }
            }
        }
    }
    visible
}

fn next_state(seats: &Vec<Vec<Position>>) -> (Vec<Vec<Position>>, bool) {
    let mut result = seats.clone();
    let mut changed = false;
    for row in 0..seats.len() {
        for col in 0..seats[0].len() {
            let pos = &seats[row][col];
            let next = match pos {
                Position::Floor => Position::Floor,
                Position::Empty => {
                    if count_adjacent(seats, row, col) == 0 {
                        Position::Occupied
                    } else {
                        Position::Empty
                    }
                },
                Position::Occupied => {
                    if count_adjacent(seats, row, col) >= 4 {
                        Position::Empty
                    } else {
                        Position::Occupied
                    }
                }
            };
            result[row][col] = next;
            changed |= seats[row][col] != result[row][col];
        }
    }
    (result, changed)
}

fn next_state_2(seats: &Vec<Vec<Position>>) -> (Vec<Vec<Position>>, bool) {
    let mut result = seats.clone();
    let mut changed = false;
    for row in 0..seats.len() {
        for col in 0..seats[0].len() {
            let pos = &seats[row][col];
            let next = match pos {
                Position::Floor => Position::Floor,
                Position::Empty => {
                    if count_visible(seats, row, col) == 0 {
                        Position::Occupied
                    } else {
                        Position::Empty
                    }
                },
                Position::Occupied => {
                    if count_visible(seats, row, col) >= 5 {
                        Position::Empty
                    } else {
                        Position::Occupied
                    }
                }
            };
            result[row][col] = next;
            changed |= seats[row][col] != result[row][col];
        }
    }
    (result, changed)
}

fn main() {
    let mut seats: Vec<Vec<_>> = INPUT.trim()
        .lines()
        .map(|l| l.chars().map(|c| Position::from(c)).collect())
        .collect();
    loop {
        let res = next_state(&seats);
        if !res.1 {
            break;
        }
        seats = res.0;
    }
    println!("Occupied count: {}",
        seats.iter().map(|r| r.iter().filter(|c| **c == Position::Occupied).count()).sum::<usize>());

    let mut seats: Vec<Vec<_>> = INPUT.trim()
        .lines()
        .map(|l| l.chars().map(|c| Position::from(c)).collect())
        .collect();
    loop {
        let res = next_state_2(&seats);
        if !res.1 {
            break;
        }
        seats = res.0;
    }
    println!("Occupied count 2: {}",
        seats.iter().map(|r| r.iter().filter(|c| **c == Position::Occupied).count()).sum::<usize>());
}

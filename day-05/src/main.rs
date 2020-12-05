const INPUT: &str = include_str!("../input");

fn binary_search(low: i32, high: i32, steps: &mut dyn Iterator<Item = i32>) -> i32 {
    let mut low = low;
    let mut high = high;
    for direction in steps {
        let m = (low + high) / 2;
        if direction == -1 {
            high = m - 1;
        } else {
            low = m + 1;
        }
    }
    low
}

fn seat_id(pass: &str) -> i32 {
    let row = binary_search(
        0,
        127,
        &mut pass.chars().take(7).map(|c| if c == 'F' { -1 } else { 1 }),
    );
    let col = binary_search(
        0,
        7,
        &mut pass.chars().skip(7).map(|c| if c == 'L' { -1 } else { 1 }),
    );
    row * 8 + col
}

fn main() {
    println!(
        "Highest seat ID: {}",
        INPUT.lines().map(seat_id).max().unwrap()
    );

    let mut all_seats: Vec<_> = INPUT.lines().map(seat_id).collect();
    all_seats.sort();

    for (a, b) in all_seats.iter().zip(all_seats.iter().skip(1)) {
        if a + 2 == *b {
            println!("My seat: {}", a + 1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::seat_id;

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}

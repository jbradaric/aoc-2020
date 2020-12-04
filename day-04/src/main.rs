use std::collections::HashSet;

const INPUT: &str = include_str!("../input");

fn field_valid(name: &str, value: &str) -> bool {
    match name {
        "byr" => {
            let v: i32 = value.parse().unwrap();
            1920 <= v && v <= 2002
        }
        "iyr" => {
            let v: i32 = value.parse().unwrap();
            2010 <= v && v <= 2020
        }
        "eyr" => {
            let v: i32 = value.parse().unwrap();
            2020 <= v && v <= 2030
        }
        "hgt" => {
            if value.ends_with("cm") {
                let v: i32 = value.strip_suffix("cm").unwrap().parse().unwrap();
                150 <= v && v <= 193
            } else if value.ends_with("in") {
                let v: i32 = value.strip_suffix("in").unwrap().parse().unwrap();
                59 <= v && v <= 76
            } else {
                false
            }
        }
        "hcl" => {
            let (p, v) = value.split_at(1);
            if p != "#" || v.len() != 6 {
                false
            } else {
                v.chars().all(|c| ('0' <= c && c <= '9') || ('a' <= c && c <= 'f'))
            }
        }
        "ecl" => {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .find(|v| v == &&value).is_some()
        }
        "pid" => {
            value.len() == 9 && value.chars().all(|c| '0' <= c && c <= '9')
        }
        "cid" => {
            true
        }
        _ => { false }
    }
}

fn passport_valid(passport: &[&str], req_fields: &HashSet<&str>) -> bool {
    let mut fields = HashSet::new();
    for line in passport.iter() {
        for field in line.split_whitespace() {
            let tmp: Vec<_> = field.split(':').collect();
            if !field_valid(tmp[0], tmp[1]) {
                return false;
            }
            fields.insert(tmp[0]);
        }
    }
    req_fields.is_subset(&fields)
}

fn main() {
    let req_fields: HashSet<_> = [
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
    ].iter().cloned().collect();
    let lines: Vec<_> = INPUT.lines().collect();
    let mut count = 0;
    for passport in lines.as_slice().split(|l| l == &"") {
        if passport_valid(passport, &req_fields) {
            count += 1;
        }
    }

    println!("Num valid passports: {}", count);
}

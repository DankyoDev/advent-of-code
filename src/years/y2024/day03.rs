use crate::errors::AocError;
use crate::models::DayResult;
use crate::utils::openfiles::open_input;

pub fn run() -> Result<DayResult, AocError> {
    let input = open_input(2024, 3)?;

    let re = regex::Regex::new(r"don't\(\)|do\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;

    let mut enabled = true;

    for cap in re.captures_iter(&input) {
        let m = cap.get(0).unwrap().as_str();

        if m.starts_with("mul(") {
            let x: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let y: i32 = cap.get(2).unwrap().as_str().parse().unwrap();

            part1 += x * y;

            if enabled {
                part2 += x * y;
            }
        } else if m == "don't()" {
            enabled = false;
        } else if m == "do()" {
            enabled = true;
        }
    }

    Ok(DayResult::new(part1.to_string(), part2.to_string()))
}

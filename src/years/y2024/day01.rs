use crate::errors::AocError;
use crate::models::DayResult;
use crate::utils::openfiles::open_input;

pub fn run() -> Result<DayResult, AocError> {
    let input = open_input(2024, 1)?;

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        if parts.len() != 2 {
            continue;
        }

        let l = parts[0].trim().parse::<i32>().unwrap();
        let r = parts[1].trim().parse::<i32>().unwrap();

        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let total_diff: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    let sim_score: i32 = left
        .iter()
        .map(|l| l * right.iter().filter(|&&r| r == *l).count() as i32)
        .sum();

    Ok(DayResult::new(
        total_diff.to_string(),
        sim_score.to_string(),
    ))
}

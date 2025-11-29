use crate::errors::AocError;
use crate::models::DayResult;
use crate::utils::openfiles::open_input;

fn is_safe(seq: &[i32]) -> bool {
    if seq.len() <= 1 {
        return true;
    }

    let mut sign = 0;

    for w in seq.windows(2) {
        let diff = w[1] - w[0];

        if diff == 0 {
            return false;
        }

        let abs = diff.abs();
        if !(1..=3).contains(&abs) {
            return false;
        }

        let cur = diff.signum();
        if sign == 0 {
            sign = cur;
        } else if cur != sign {
            return false;
        }
    }

    true
}

fn is_safe_with_one_removed(seq: &[i32]) -> bool {
    if is_safe(seq) {
        return true;
    }

    if seq.len() <= 2 {
        return true;
    }

    for skip in 0..seq.len() {
        let mut reduced = Vec::with_capacity(seq.len() - 1);
        for (i, val) in seq.iter().enumerate() {
            if i != skip {
                reduced.push(*val);
            }
        }

        if is_safe(&reduced) {
            return true;
        }
    }

    false
}

pub fn run() -> Result<DayResult, AocError> {
    let input = open_input(2024, 2)?;

    let mut safe_part1 = 0;
    let mut safe_part2 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let seq: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        if is_safe(&seq) {
            safe_part1 += 1;
        }

        if is_safe_with_one_removed(&seq) {
            safe_part2 += 1;
        }
    }

    Ok(DayResult::new(
        safe_part1.to_string(),
        safe_part2.to_string(),
    ))
}

use crate::errors::AocError;
use crate::models::DayResult;
use crate::utils::openfiles::open_input;

pub fn run() -> Result<DayResult, AocError> {
    let input = open_input(2025, 3)?;

    let mut part1: u128 = 0;
    let mut part2: u128 = 0;

    for line in input.lines() {
        let s = line.trim();
        if s.is_empty() {
            continue;
        }

        let bytes = s.as_bytes();
        let n = bytes.len();

        if n >= 2 {
            // best_after[i] = dígito máximo en posiciones > i (None si no hay)
            let mut best_after: Vec<Option<u8>> = vec![None; n];
            let mut cur_max: Option<u8> = None;
            for i in (0..n).rev() {
                best_after[i] = cur_max;
                let d = bytes[i] - b'0';
                cur_max = match cur_max {
                    Some(m) if m > d => Some(m),
                    _ => Some(d),
                };
            }

            let mut best_val: u8 = 0;
            for i in 0..n - 1 {
                if let Some(b) = best_after[i] {
                    let a = bytes[i] - b'0';
                    let val = a * 10 + b;
                    if val > best_val {
                        best_val = val;
                    }
                }
            }

            part1 += best_val as u128;
        }

        const K: usize = 12;
        if n >= K {
            let mut stack: Vec<u8> = Vec::with_capacity(K);
            let mut to_remove = n - K;

            for &b in bytes {
                let d = b - b'0';
                while !stack.is_empty() && to_remove > 0 && *stack.last().unwrap() < d {
                    stack.pop();
                    to_remove -= 1;
                }
                stack.push(d);
            }

            stack.truncate(K);

            let mut val: u128 = 0;
            for d in stack {
                val = val * 10 + d as u128;
            }

            part2 += val;
        }
    }

    Ok(DayResult::new(part1.to_string(), part2.to_string()))
}

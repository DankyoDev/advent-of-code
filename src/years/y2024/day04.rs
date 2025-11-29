use crate::errors::AocError;
use crate::models::DayResult;
use crate::utils::openfiles::open_input;

pub fn run() -> Result<DayResult, AocError> {
    let input = open_input(2024, 4)?;

    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();

    let h = grid.len();
    let w = grid[0].len();

    // ----- Part 1 -----
    const WORD: &[u8] = b"XMAS";
    const DIRS: &[(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut part1 = 0;

    for r in 0..h {
        for c in 0..w {
            if grid[r][c] != b'X' {
                continue;
            }

            for &(dr, dc) in DIRS {
                let mut ok = true;
                for step in 0..4 {
                    let nr = r as isize + dr * step as isize;
                    let nc = c as isize + dc * step as isize;

                    if nr < 0 || nc < 0 || nr >= h as isize || nc >= w as isize {
                        ok = false;
                        break;
                    }

                    if grid[nr as usize][nc as usize] != WORD[step] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    part1 += 1;
                }
            }
        }
    }

    // ----- Part 2 -----
    let mut part2 = 0;

    for r in 1..h - 1 {
        for c in 1..w - 1 {
            if grid[r][c] != b'A' {
                continue;
            }

            let d1 = [grid[r - 1][c - 1], b'A', grid[r + 1][c + 1]];
            let d2 = [grid[r - 1][c + 1], b'A', grid[r + 1][c - 1]];

            let mas = [b'M', b'A', b'S'];
            let sam = [b'S', b'A', b'M'];

            let ok1 = d1 == mas || d1 == sam;
            let ok2 = d2 == mas || d2 == sam;

            if ok1 && ok2 {
                part2 += 1;
            }
        }
    }

    Ok(DayResult::new(part1.to_string(), part2.to_string()))
}

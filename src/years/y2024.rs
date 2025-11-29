mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use crate::errors::AocError;

pub fn run_day(day: u8) -> Result<(), AocError> {
    let result = match day {
        1 => day01::run()?,
        2 => day02::run()?,
        3 => day03::run()?,
        4 => day04::run()?,
        5 => day05::run()?,
        6 => day06::run()?,
        7 => day07::run()?,
        _ => return Err(AocError::Unsupported),
    };

    println!("Respuesta 1: {}", result.part1);
    println!("Respuesta 2: {}", result.part2);

    Ok(())
}

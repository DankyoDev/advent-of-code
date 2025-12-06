pub mod y2024;
pub mod y2025;

use crate::errors::AocError;

pub fn dispatch(year: u16, day: u8) -> Result<(), AocError> {
    match year {
        2024 => y2024::run_day(day),
        2025 => y2025::run_day(day),
        _ => Err(AocError::Unsupported),
    }
}

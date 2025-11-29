use std::fs;
use std::path::PathBuf;

pub fn open_input(year: u16, day: u8) -> Result<String, std::io::Error> {
    let day_str = format!("{:02}", day);

    let mut path = PathBuf::from("inputs");
    path.push(format!("y{}", year));
    path.push(format!("{}.txt", day_str));

    fs::read_to_string(path)
}

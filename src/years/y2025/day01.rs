use crate::errors::AocError;
use crate::models::DayResult;
use crate::utils::openfiles::open_input;

pub fn run() -> Result<DayResult, AocError> {
  let input = open_input(2025, 1)?;
  
  let mut pos: i32 = 50;
  let mut p1: usize = 0;
  let mut p2: usize = 0;
  
  for line in input.lines() {
	let line = line.trim();
	if line.is_empty() {
	  continue;
	}
	
	let mut chars = line.chars();
	let dir = chars.next().unwrap_or(' ');
	let dist: i32 = chars.as_str().trim().parse().unwrap_or(0);
	
	let steps = ((dist % 10000) + 10000) % 10000; // maneja grandes
	
	match dir {
	  'R' => {
		for i in 1..=steps {
		  let np = (pos + i) % 100;
		  if np == 0 { p2 += 1; }
		}
		pos = (pos + steps) % 100;
	  }
	  'L' => {
		for i in 1..=steps {
		  let np = (pos - i) % 100;
		  let np = if np < 0 { np + 100 } else { np };
		  if np == 0 { p2 += 1; }
		}
		pos = (pos - steps) % 100;
		if pos < 0 { pos += 100; }
	  }
	  _ => {}
	}
	
	if pos == 0 { p1 += 1; }
  }
  
  Ok(DayResult::new(p1.to_string(), p2.to_string()))
}

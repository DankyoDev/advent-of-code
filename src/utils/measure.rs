use std::time::{Instant};

pub fn run_timed<F, T>(show: bool, f: F) -> T
where
  F: FnOnce() -> T,
{
  if !show {
	return f();
  }
  
  let start = Instant::now();
  let result = f();
  let elapsed = start.elapsed();
  
  println!("Tiempo: {}.{:03}s",
		   elapsed.as_secs(),
		   elapsed.subsec_millis()
  );
  
  result
}

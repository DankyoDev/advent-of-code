use crate::errors::AocError;
use crate::models::DayResult;
use crate::utils::openfiles::open_input;

use rustc_hash::FxHashSet;

#[inline]
fn pow10_digits(x: i128) -> i128 {
  if x == 0 { return 10; }
  let d = x.abs().ilog10() + 1;
  10_i128.pow(d as u32)
}

#[inline]
fn concat(a: i128, b: i128) -> Option<i128> {
  a.checked_mul(pow10_digits(b))?.checked_add(b)
}

#[inline]
fn solve(nums: &[i128], target: i128, allow_concat: bool) -> bool {
  let positive = nums.iter().all(|&n| n >= 0);
  let mut cur: FxHashSet<i128> = FxHashSet::with_capacity_and_hasher(64, Default::default());
  cur.insert(nums[0]);
  
  for &n in &nums[1..] {
	let mut next: FxHashSet<i128> =
	  FxHashSet::with_capacity_and_hasher(cur.len() * if allow_concat {3} else {2}, Default::default());
	
	for &v in cur.iter() {
	  if let Some(x) = v.checked_add(n) { if !positive || x <= target { next.insert(x); }}
	  if let Some(x) = v.checked_mul(n) { if !positive || x <= target { next.insert(x); }}
	  
	  if allow_concat {
		if let Some(x) = concat(v,n) { if !positive || x <= target { next.insert(x); }}
	  }
	}
	
	if next.is_empty() { return false; }
	cur = next;
  }
  
  cur.contains(&target)
}

pub fn run() -> Result<DayResult, AocError> {
  use rayon::prelude::*;
  
  let input = open_input(2024, 7)?;
  let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
  
  let (part1, part2) = lines
	.par_iter()
	.map(|line| {
	  let (lhs, rhs) = line.split_once(':').unwrap();
	  let target: i128 = lhs.trim().parse().unwrap();
	  let nums: Vec<i128> = rhs.split_whitespace().map(|x| x.parse().unwrap()).collect();
	  
	  let ok1 = solve(&nums, target, false);
	  let ok2 = if ok1 { true } else { solve(&nums, target, true) };
	  
	  (if ok1 { target } else { 0 }, if ok2 { target } else { 0 })
	})
	.reduce(|| (0,0), |a,b| (a.0+b.0, a.1+b.1));
  
  Ok(DayResult::new(part1.to_string(), part2.to_string()))
}


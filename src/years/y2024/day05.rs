use crate::errors::AocError;
use crate::models::DayResult;
use crate::utils::openfiles::open_input;

use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() -> Result<DayResult, AocError> {
    let input = open_input(2024, 5)?;

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut parsing_rules = true;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let (a, b) = line.split_once('|').unwrap();
            rules.push((a.parse().unwrap(), b.parse().unwrap()));
        } else {
            let pages = line
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            updates.push(pages);
        }
    }

    let mut graph_all: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (a, b) in &rules {
        graph_all.entry(*a).or_default().insert(*b);
    }

    fn valid(update: &[i32], g: &HashMap<i32, HashSet<i32>>) -> bool {
        let pos: HashMap<i32, usize> = update.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        for (&a, bs) in g {
            if let Some(&pa) = pos.get(&a) {
                for &b in bs {
                    if let Some(&pb) = pos.get(&b) {
                        if pb < pa {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn reorder(update: &[i32], g: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
        let set: HashSet<i32> = update.iter().copied().collect();

        let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut indeg: HashMap<i32, usize> = update.iter().map(|&x| (x, 0)).collect();

        for (&a, bs) in g {
            if !set.contains(&a) {
                continue;
            }
            for &b in bs {
                if set.contains(&b) {
                    adj.entry(a).or_default().insert(b);
                    *indeg.get_mut(&b).unwrap() += 1;
                }
            }
        }

        let mut q: VecDeque<i32> = indeg
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(n, _)| *n)
            .collect();

        let mut out = Vec::new();
        while let Some(x) = q.pop_front() {
            out.push(x);
            if let Some(v) = adj.get(&x) {
                for &nx in v {
                    let c = indeg.get_mut(&nx).unwrap();
                    *c -= 1;
                    if *c == 0 {
                        q.push_back(nx);
                    }
                }
            }
        }
        out
    }

    let mut sum_ok = 0;
    let mut sum_fix = 0;

    for upd in &updates {
        if valid(upd, &graph_all) {
            sum_ok += upd[upd.len() / 2];
        } else {
            let f = reorder(upd, &graph_all);
            sum_fix += f[f.len() / 2];
        }
    }

    Ok(DayResult::new(sum_ok.to_string(), sum_fix.to_string()))
}

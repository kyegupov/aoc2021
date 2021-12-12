use itertools::Itertools as _;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;

fn solve(
    cache: &mut BTreeMap<(String, BTreeMap<String, usize>), i64>,
    edges: &BTreeMap<String, BTreeSet<String>>,
    start: String,
    visited_small: BTreeMap<String, usize>,
) -> i64 {
    let cached = cache.get(&(start.clone(), visited_small.clone()));
    if let Some(x) = cached {
        return *x;
    }
    let mut count = 0i64;
    let had_2 = visited_small.values().filter(|x| x == &&2).count() > 1;
    let limit = if had_2 { 1 } else { 2 };
    for n in edges[&start]
        .iter()
        .filter(|x| visited_small.get(x.clone()).unwrap_or(&0) < &limit)
    {
        if n == "end" {
            count += 1;
        } else {
            let mut v2 = visited_small.clone();
            if n.chars().next().unwrap().is_lowercase() {
                *v2.entry(n.clone()).or_insert(0) += 1;
            }
            count += solve(cache, edges, n.to_string(), v2);
        }
    }
    cache.insert((start.clone(), visited_small.clone()), count);
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input12.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let mut edges = BTreeMap::new();
    for l in lines {
        let parts: Vec<&str> = l.split("-").collect();
        edges
            .entry(parts[0].to_string())
            .or_insert_with(BTreeSet::new)
            .insert(parts[1].to_string());
        edges
            .entry(parts[1].to_string())
            .or_insert_with(BTreeSet::new)
            .insert(parts[0].to_string());
    }
    let mut solutions: BTreeMap<(String, BTreeMap<String, usize>), i64> = BTreeMap::new();

    let mut visited = BTreeMap::new();
    visited.insert("start".into(), 2);

    let res = solve(&mut solutions, &edges, "start".to_string(), visited);

    // dbg!(solutions);

    println!("{}", res);
    Ok(())
}

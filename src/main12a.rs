use itertools::Itertools as _;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;

fn solve(
    cache: &mut BTreeMap<(String, BTreeSet<String>), i64>,
    edges: &BTreeMap<String, BTreeSet<String>>,
    start: String,
    visited_small: BTreeSet<String>,
) -> i64 {
    let cached = cache.get(&(start.clone(), visited_small.clone()));
    if let Some(x) = cached {
        return *x;
    }
    let mut count = 0i64;
    for n in edges[&start]
        .iter()
        .filter(|x| !visited_small.contains(x.clone()))
    {
        if n == "end" {
            count += 1;
        } else {
            let mut v2 = visited_small.clone();
            if n.chars().next().unwrap().is_lowercase() {
                v2.insert(n.clone());
            }
            count += solve(cache, edges, n.to_string(), v2);
        }
    }
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
    let mut solutions: BTreeMap<(String, BTreeSet<String>), i64> = BTreeMap::new();

    let mut visited = BTreeSet::new();
    visited.insert("start".into());

    let res = solve(&mut &mut solutions, &edges, "start".to_string(), visited);

    println!("{}", res);
    Ok(())
}

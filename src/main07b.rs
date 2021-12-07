use itertools::Itertools as _;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input07.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let mut crabs: Vec<i64> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    let f = |x: i64| x * (x + 1) / 2;

    let mut range = (*crabs.iter().min().unwrap()..=*crabs.iter().max().unwrap());

    let res: i64 = range
        .clone()
        .map(|x| crabs.iter().map(|c| f((c - x).abs())).sum())
        .min()
        .unwrap();

    println!("{}", res);
    Ok(())
}

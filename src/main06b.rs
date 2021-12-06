use itertools::Itertools as _;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input06.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let mut fishes: BTreeMap<i64, i64> = BTreeMap::new();
    for i in lines[0].split(',').map(|x| x.parse().unwrap()) {
        *fishes.entry(i).or_insert(0) += 1;
    }

    for _ in 0..256 {
        let new = *fishes.get(&0).unwrap_or(&0);
        for i in 0..8 {
            fishes.insert(i, *fishes.get(&(i + 1)).unwrap_or(&0));
        }
        *fishes.get_mut(&6).unwrap() += new;
        fishes.insert(8, new);
        dbg!(&fishes);
    }

    let res: i64 = fishes.values().sum();

    println!("{}", res);
    Ok(())
}

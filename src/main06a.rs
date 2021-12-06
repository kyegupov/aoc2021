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

    let mut fishes: Vec<i64> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    for _ in 0..80 {
        let mut add = 0;
        for f in fishes.iter_mut() {
            *f -= 1;
            if *f < 0 {
                *f = 6;
                add += 1;
            }
        }
        for _ in 0..add {
            fishes.push(8);
        }
    }

    let res = fishes.len();

    println!("{}", res);
    Ok(())
}

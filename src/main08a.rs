use itertools::Itertools as _;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input08.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let mut res = 0;

    for l in lines {
        let out = l.split(" | ").skip(1).next().unwrap().split_whitespace();
        for w in out {
            match w.len() {
                2 | 3 | 4 | 7 => {
                    res += 1;
                }
                _ => {}
            }
        }
    }

    println!("{}", res);
    Ok(())
}

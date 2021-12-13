use itertools::Itertools as _;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input13.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let mut a = BTreeSet::new();
    for l in lines.iter().take_while(|l| !l.is_empty()) {
        let parts: Vec<isize> = l.split(",").map(|x| x.parse().unwrap()).collect_vec();
        a.insert((parts[0], parts[1]));
    }

    // for y in 0..a.iter().map(|x| x.1).max().unwrap() {
    //     for x in 0..a.iter().map(|x| x.0).max().unwrap() {
    //         print!("{}", if a.contains(&(x, y)) { "#" } else { "." });
    //     }
    //     println!();
    // }
    // println!();

    for l in lines.iter().filter(|l| l.starts_with("fold")) {
        let parts = l.split(" ").nth(2).unwrap().split('=').collect_vec();
        let cut: isize = parts[1].parse().unwrap();
        if parts[0] == "y" {
            for xy in a.iter().copied().collect_vec() {
                if xy.1 > cut {
                    a.insert((xy.0, 2 * cut - xy.1));
                }
            }
            a.retain(|xy| xy.1 < cut)
        }
        if parts[0] == "x" {
            for xy in a.iter().copied().collect_vec() {
                if xy.0 > cut {
                    a.insert((2 * cut - xy.0, xy.1));
                }
            }
            a.retain(|xy| xy.0 < cut)
        }
    }

    for y in 0..=a.iter().map(|x| x.1).max().unwrap() {
        for x in 0..=a.iter().map(|x| x.0).max().unwrap() {
            print!("{}", if a.contains(&(x, y)) { "#" } else { "." });
        }
        println!();
    }

    let res = a.len();

    println!("{}", res);
    Ok(())
}

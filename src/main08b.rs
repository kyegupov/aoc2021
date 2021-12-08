#[macro_use]
extern crate maplit;

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

    let digits = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    let digits_set: BTreeSet<String> = digits.iter().map(|x| x.to_string()).collect();
    let digits_vec: Vec<String> = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();

    let base: BTreeMap<char, usize> = "abcdefg".chars().enumerate().map(|(a, b)| (b, a)).collect();

    for l in lines {
        let mut parts = l.split(" | ");
        let digs: Vec<&str> = parts.next().unwrap().split_whitespace().collect();
        let out: Vec<&str> = parts.next().unwrap().split_whitespace().collect();
        for p in base.keys().permutations(7) {
            let permuted: BTreeSet<String> = digs
                .iter()
                .map(|w| w.chars().map(|c| p[(c as usize)-('a' as usize)]).sorted().join(""))
                .collect();
            if permuted == digits_set {
                let permuted_out: Vec<String> = out
                    .iter()
                    .map(|w| w.chars().map(|c| p[(c as usize)-('a' as usize)]).sorted().join(""))
                    .collect();
                let mut x = 0i64;
                for w in permuted_out {
                    x *= 10;
                    x += digits_vec.iter().position(|x| x == &w).unwrap() as i64;
                }
                res += x;
            }
        }
    }

    println!("{}", res);
    Ok(())
}

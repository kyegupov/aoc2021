use std::error::Error;
use std::fs::{read_to_string};
use itertools::Itertools as _;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input03.txt")?.lines().map(|x|x.into()).collect();
    let mut freq = (0..lines[0].len()).map(|_|0usize).collect_vec();

    for l in &lines {
        for (i, c) in l.chars().enumerate() {
            freq[i] += if c == '1' {1} else {0};
        }
    }
    let gamma = i64::from_str_radix(&freq.iter().map(|x| if *x > lines.len()/2 {"1"} else {"0"}).join(""), 2).unwrap();
    let epsilon = i64::from_str_radix(&freq.iter().map(|x| if *x > lines.len()/2 {"0"} else {"1"}).join(""), 2).unwrap();

    let res = gamma * epsilon;

    println!("{}", res);
    Ok(())
}
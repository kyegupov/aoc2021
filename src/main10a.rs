use itertools::Itertools as _;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input10.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let open = "([{<";

    let score = [3, 57, 1197, 25137];

    let mut res = 0;

    for l in lines {
        let mut stack = vec![];
        for c in l.chars() {
            if open.contains(c) {
                stack.push(c);
            } else {
                let pos = ")]}>".find(c).unwrap();
                let oc = open.chars().skip(pos).next().unwrap();
                if stack.pop().unwrap() != oc {
                    res += score[pos];
                    break;
                }
            }
        }
    }

    println!("{}", res);
    Ok(())
}

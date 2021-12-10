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
    let mut scores = vec![];

    for l in lines {
        let mut stack = vec![];
        let mut bad = false;
        for c in l.chars() {
            if open.contains(c) {
                stack.push(c);
            } else {
                let pos = ")]}>".find(c).unwrap();
                let oc = open.chars().skip(pos).next().unwrap();
                if stack.pop().unwrap() != oc {
                    bad = true;
                    break;
                }
            }
        }
        let mut scor = 0i64;
        if !bad {
            while !stack.is_empty() {
                let c = stack.pop().unwrap();
                let pos = open.find(c).unwrap();
                scor *= 5;
                scor += pos as i64 + 1;
            }
            scores.push(scor);
        }
    }

    scores.sort();
    res = scores[scores.len() / 2];

    println!("{}", res);
    Ok(())
}

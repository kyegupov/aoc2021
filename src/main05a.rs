use itertools::Itertools as _;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input05.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let mut board: BTreeMap<(usize, usize), i64> = BTreeMap::new();

    for l in &lines {
        let (s, f): ((usize, usize), (usize, usize)) = l
            .split(" -> ")
            .map(|x| {
                x.split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        if s.0 == f.0 {
            dbg!((&s, &f));
            for i in min(s.1, f.1)..=max(s.1, f.1) {
                *board.entry((s.0, i)).or_default() += 1;
            }
        }
        if s.1 == f.1 {
            dbg!((&s, &f));
            for i in min(s.0, f.0)..=max(s.0, f.0) {
                *board.entry((i, s.1)).or_default() += 1;
            }
        }
    }
    // dbg!(&board);
    let res = board.values().filter(|v| **v > 1).count();
    println!("{}", res);
    Ok(())
}

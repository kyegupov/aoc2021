use itertools::Itertools as _;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet, LinkedList};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input14.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let mut paircount: BTreeMap<(char, char), i64> = BTreeMap::new(); //linked list
    for (c1, c2) in lines[0].chars().tuple_windows() {
        *paircount.entry((c1, c2)).or_insert(0) += 1;
    }

    let mut subs = BTreeMap::new();

    for l in lines[2..].iter() {
        let parts: Vec<&str> = l.split(" -> ").collect_vec();
        subs.insert(
            (
                parts[0].chars().nth(0).unwrap(),
                parts[0].chars().nth(1).unwrap(),
            ),
            parts[1].chars().nth(0).unwrap(),
        );
    }

    for _ in 0..40 {
        let mut adds: BTreeMap<(char, char), i64> = BTreeMap::new();
        for (pair, &ins) in subs.iter() {
            if let Some(out) = paircount.remove(pair) {
                *adds.entry((pair.0, ins)).or_insert(0) += out;
                *adds.entry((ins, pair.1)).or_insert(0) += out;
            }
        }
        for (k, v) in adds.iter() {
            *paircount.entry(*k).or_insert(0) += *v;
        }
    }

    let mut stats: BTreeMap<_, i64> = BTreeMap::new();
    for (k, v) in paircount {
        // *stats.entry(k.0).or_default() += v;
        *stats.entry(k.1).or_default() += v;
    }
    *stats.get_mut(&lines[0].chars().next().unwrap()).unwrap() += 1;

    dbg!(&stats);

    let res: i64 = *stats.values().max().unwrap() - *stats.values().min().unwrap();
    println!("{}", res);
    Ok(())
}

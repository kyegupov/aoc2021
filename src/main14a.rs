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

    let mut a: BTreeMap<usize, (char, usize)> = BTreeMap::new(); //linked list
    for c in lines[0].chars() {
        a.insert(a.len(), (c, a.len() + 1));
    }
    a.insert(a.len() - 1, (a[&(a.len() - 1)].0, 9999999999));
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

    for _ in 0..10 {
        let mut i = 0;
        let mut p = ' ';
        while a.contains_key(&i) {
            let (c, i2) = a[&i];
            if subs.contains_key(&(p, c)) {
                let i3 = a.len();
                a.insert(i, (subs[&(p, c)], i3));
                a.insert(i3, (c, i2));
            }
            p = c;
            i = i2;
        }

        let mut i = 0;
        while a.contains_key(&i) {
            print!("{}", a[&i].0);
            i = a[&i].1;
        }
        println!();
    }

    let mut stats = BTreeMap::new();
    let mut i = 0;
    while a.contains_key(&i) {
        *stats.entry(a[&i].0).or_insert(0) += 1;
        i = a[&i].1;
    }

    let res = *stats.values().max().unwrap() - *stats.values().min().unwrap();
    println!("{}", res);
    Ok(())
}

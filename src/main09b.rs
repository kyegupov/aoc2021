use itertools::Itertools as _;
use maplit::btreeset;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;
use std::slice::SliceIndex;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input09.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let mut a = BTreeMap::new();

    let mut basins: BTreeMap<(isize, isize), i32> = BTreeMap::new();
    let mut basinmap = BTreeMap::new();
    let mut nextb = 1;

    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            a.insert((x as isize,y as isize), c.to_string().parse().unwrap());
        }
    }

    let dirs = [(1isize,0isize),(0,1),(-1,0),(0,-1)];

    let mut res = 0;

    for y in 0..lines.len() as isize {
        for x in 0..lines[0].len() as isize {
            let c: i64 = a[&(x,y)];
            if c == 9 {
                continue;
            }
            let matching_basins = (basins.get(&(x-1,y)).copied(), basins.get(&(x,y-1)).copied());
            match matching_basins {
                (Some(a), Some(b)) => {
                    basinmap.insert(max(a,b), min(a, b));
                    basins.insert((x,y), min(a,b));
                },
                (Some(a), None) => {basins.insert((x,y), a);},
                (None, Some(a)) => {basins.insert((x,y), a);},
                (None, None) => {basins.insert((x,y), nextb); basinmap.insert(nextb, nextb); nextb += 1;},
            }
        }
    }

    let mut sizes = BTreeMap::new();
    for y in 0..lines.len() as isize {
        for x in 0..lines[0].len() as isize {
            let mut bb = *basins.get(&(x,y)).unwrap_or(&0);
            while bb > 0 && basinmap[&bb] != bb {
                bb = basinmap[&bb];
            }
            print!("{:4}", bb);
        }
        println!();
    }

    for (k, b) in basins.iter() {
        let mut bb = *b;
        while basinmap[&bb] != bb {
            bb = basinmap[&bb];
        }
        *sizes.entry(bb).or_insert(0) += 1;
    }
    let mut szs: Vec<_> = sizes.values().collect();
    szs.sort();
    szs.reverse();

    let res = szs[0]*szs[1]*szs[2];

    println!("{}", res);
    Ok(())
}

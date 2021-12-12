use itertools::Itertools as _;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input11.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let mut a = BTreeMap::new();
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            a.insert((x as i64, y as i64), c as i64 - '0' as i64);
        }
    }

    let mut res = 0;

    for _ in 0..100 {
        for cp in a.values_mut() {
            *cp += 1;
        }
        // for y in 0..10 {
        //     for x in 0..10 {
        //         print!("{}", a[&(x, y)]);
        //     }
        //     println!();
        // }
        let mut flashed = BTreeSet::new();
        loop {
            let mut new_flashes = BTreeSet::new();
            for (&xy, cp) in a.iter_mut() {
                if *cp > 9 {
                    *cp = 0;
                    if !flashed.contains(&xy) {
                        flashed.insert(xy);
                        new_flashes.insert(xy);
                        res += 1;
                    }
                }
            }
            for xy in new_flashes.iter() {
                for dy in -1i64..=1 {
                    for dx in -1i64..=1 {
                        if dx != 0 || dy != 0 {
                            let xy2 = (xy.0 + dx, xy.1 + dy);
                            if !flashed.contains(&xy2) {
                                a.entry(xy2).and_modify(|e| *e += 1);
                            }
                        }
                    }
                }
            }
            // dbg!(&new_flashes);
            // for y in 0..10 {
            //     for x in 0..10 {
            //         print!("{}", a[&(x, y)]);
            //     }
            //     println!();
            // }

            if new_flashes.is_empty() {
                break;
            }
        }
        // for y in 0..10 {
        //     for x in 0..10 {
        //         print!("{}", a[&(x, y)]);
        //     }
        //     println!();
        // }
        // println!();
    }

    println!("{}", res);
    Ok(())
}

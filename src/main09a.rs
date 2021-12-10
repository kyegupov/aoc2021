use itertools::Itertools as _;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input09.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let mut a = BTreeMap::new();

    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            a.insert((x as isize, y as isize), c.to_string().parse().unwrap());
        }
    }

    let dirs = [(1isize, 0isize), (0, 1), (-1, 0), (0, -1)];

    let mut res = 0;

    for y in 0..lines.len() as isize {
        for x in 0..lines[0].len() as isize {
            let mut lowest = true;
            for d in dirs {
                let xx = x + d.0;
                let yy = y + d.1;
                if a.get(&(xx, yy)).unwrap_or(&9999) <= &a[&(x, y)] {
                    lowest = false;
                    break;
                }
            }
            if lowest {
                // dbg!(x,y,a[&(x,y)]);
                res += 1 + a[&(x, y)];
            }
        }
    }

    println!("{}", res);
    Ok(())
}

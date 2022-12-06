use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{alpha1, digit0};
use nom::combinator::{map, map_res};
use nom::error;
use nom::sequence::{preceded, terminated, tuple};
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
use std::error::Error;
use std::fs::read_to_string;
use std::{env, fmt};

extern crate nom;
fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");
    let number = |x| {
        map_res(is_a::<_, _, error::Error<_>>("-0123456789"), |y: &str| {
            y.parse::<i64>()
        })(x)
    };
    let range = |x| map(tuple((number, preceded(tag(".."), number))), |(x, y)| x..=y)(x);
    let coords = |x| {
        tuple((
            preceded(tag("x="), range),
            preceded(tag(",y="), range),
            preceded(tag(",z="), range),
        ))(x)
    };
    let rule = |x| tuple((map(terminated(alpha1, tag(" ")), |x| x == "on"), coords))(x);

    let mut rules = vec![];
    for (i, line) in read_to_string("input22.txt")?.lines().enumerate() {
        rules.push(rule(line).unwrap().1);
    }

    if !part2 {
        let mut space = BTreeSet::new();

        for z in -50..=50 {
            for y in -50..=50 {
                for x in -50..=50 {
                    let mut state = false;
                    for (action, ranges) in &rules {
                        if ranges.0.contains(&x) && ranges.1.contains(&y) && ranges.2.contains(&z) {
                            state = *action;
                        }
                    }
                    if state {
                        space.insert((x, y, z));
                    } else {
                        space.remove(&(x, y, z));
                    }
                }
            }
        }

        dbg!(space.len());
    } else {
        let mut blocks: BTreeMap<i64, BTreeMap<i64, BTreeSet<i64>>> = BTreeMap::new();

        let x_segs: BTreeSet<i64> = rules
            .iter()
            .map(|r| [*r.1 .0.start(), *r.1 .0.end() + 1])
            .flatten()
            .collect();
        let y_segs: BTreeSet<i64> = rules
            .iter()
            .map(|r| [*r.1 .1.start(), *r.1 .1.end() + 1])
            .flatten()
            .collect();
        let z_segs: BTreeSet<i64> = rules
            .iter()
            .map(|r| [*r.1 .2.start(), *r.1 .2.end() + 1])
            .flatten()
            .collect();
        for (action, ranges) in &rules {
            let zr = z_segs.range(ranges.2.clone());
            let yr = y_segs.range(ranges.1.clone());
            let xr = x_segs.range(ranges.0.clone());
            for &z in zr {
                let b2 = blocks.entry(z).or_default();
                for &y in yr.clone() {
                    let b3 = b2.entry(y).or_default();
                    if *action {
                        b3.extend(xr.clone());
                    } else {
                        for x in xr.clone() {
                            b3.remove(&x);
                        }
                    }
                }
            }
        }
        let mut acc = 0i64;
        for (zseg, b2) in blocks.iter() {
            let zn = *z_segs.range(zseg + 1..).next().unwrap();
            for (yseg, b3) in b2.iter() {
                let yn = *y_segs.range(yseg + 1..).next().unwrap();
                for xseg in b3.iter() {
                    let xn = *x_segs.range(xseg + 1..).next().unwrap();
                    acc += (zn - zseg) * (yn - yseg) * (xn - xseg);
                }
            }
        }
        dbg!(acc);
    }

    Ok(())
}

use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::digit0;
use nom::combinator::map;
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
use std::error::Error;
use std::fmt;
use std::fs::read_to_string;

type Orientation = ((usize, usize, usize), Coords);
type Coords = (i64, i64, i64);

extern crate nom;
fn main() -> Result<(), Box<dyn Error>> {
    let mut first = true;
    let mut pos1 = 0u64;
    let mut pos2 = 0u64;
    for l in read_to_string("input21.txt")?.lines() {
        if first {
            pos1 = l.split(":").skip(1).next().unwrap().trim().parse().unwrap();
            first = false;
        } else {
            pos2 = l.split(":").skip(1).next().unwrap().trim().parse().unwrap();
        }
    }

    let mut s1 = 0u64;
    let mut s2 = 0u64;

    let mut dice = (1..=100).cycle();
    let mut rolls = 0u64;

    loop {
        for i in 0..3 {
            pos1 += dice.next().unwrap();
            rolls += 1;
        }
        pos1 = ((pos1 - 1) % 10) + 1;
        s1 += pos1;
        if s1 >= 1000 {
            break;
        };
        for i in 0..3 {
            pos2 += dice.next().unwrap();
            rolls += 1;
        }
        pos2 = ((pos2 - 1) % 10) + 1;
        s2 += pos2;
        if s2 >= 1000 {
            break;
        };
    }
    dbg!(s1, s2, rolls);
    println!("{}", min(s1, s2) * rolls);

    Ok(())
}

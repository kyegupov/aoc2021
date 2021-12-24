use itertools::Itertools as _;
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList};
use std::error::Error;
use std::fs::read_to_string;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Cell {
    xy: (isize, isize),
    dist: i64,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.xy.cmp(&other.xy))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_num(bits: &Vec<bool>, pos: &mut usize, n: usize) -> usize {
    let mut res = 0;
    for _ in 0..n {
        res *= 2;
        res += if bits[*pos] { 1 } else { 0 };
        *pos += 1;
    }
    res
}

fn decode_packet(bits: &Vec<bool>, pos: &mut usize, sum_vers: &mut usize) {
    let v = get_num(bits, pos, 3);
    *sum_vers += v;
    let t = get_num(bits, pos, 3);
    dbg!(v, t);
    if t == 4 {
        while {
            let m = get_num(bits, pos, 1) > 0;
            *pos += 4;
            m
        } {}
    } else {
        let m = get_num(bits, pos, 1) == 0;
        dbg!(m);
        if m {
            let total = get_num(bits, pos, 15);
            let target = *pos + total;
            while *pos < target {
                decode_packet(bits, pos, sum_vers);
            }
        } else {
            let n = get_num(bits, pos, 11);
            dbg!(n);
            for _ in 0..n {
                decode_packet(bits, pos, sum_vers);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input16.txt")?
        .lines()
        .map(|x| x.into())
        .collect();
    let mut bits: Vec<bool> = vec![];

    for c in lines[0].chars() {
        let b = u8::from_str_radix(&c.to_string().to_lowercase(), 16).unwrap();
        let mut mask = 8;
        while mask > 0 {
            bits.push(b & mask > 0);
            mask >>= 1;
        }
    }

    let mut res = 0;

    decode_packet(&bits, &mut 0, &mut res);

    println!("{}", res);
    Ok(())
}

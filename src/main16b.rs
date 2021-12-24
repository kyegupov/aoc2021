use itertools::Itertools as _;
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList};
use std::error::Error;
use std::fs::read_to_string;

fn get_num(bits: &Vec<bool>, pos: &mut usize, n: usize) -> usize {
    let mut res = 0;
    for _ in 0..n {
        res *= 2;
        res += if bits[*pos] { 1 } else { 0 };
        *pos += 1;
    }
    res
}

fn decode_packet(bits: &Vec<bool>, pos: &mut usize, sum_vers: &mut usize) -> usize {
    let v = get_num(bits, pos, 3);
    *sum_vers += v;
    let t = get_num(bits, pos, 3);
    dbg!(v, t);
    if t == 4 {
        let mut res = 0;
        while {
            let m = get_num(bits, pos, 1) > 0;
            res *= 16;
            res += get_num(bits, pos, 4);
            m
        } {}
        return res;
    } else {
        let m = get_num(bits, pos, 1) == 0;
        let mut operands = vec![];
        if m {
            let total = get_num(bits, pos, 15);
            let target = *pos + total;
            while *pos < target {
                operands.push(decode_packet(bits, pos, sum_vers));
            }
        } else {
            let n = get_num(bits, pos, 11);
            dbg!(n);
            for _ in 0..n {
                operands.push(decode_packet(bits, pos, sum_vers));
            }
        }
        match t {
            0 => operands.iter().sum(),
            1 => operands.iter().product(),
            2 => *operands.iter().min().unwrap(),
            3 => *operands.iter().max().unwrap(),
            5 => {
                if operands[0] > operands[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if operands[0] < operands[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if operands[0] == operands[1] {
                    1
                } else {
                    0
                }
            }
            _ => unimplemented!(),
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

    let res = decode_packet(&bits, &mut 0, &mut 0);

    println!("{}", res);
    Ok(())
}

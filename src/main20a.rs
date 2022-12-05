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
    let mut expansion = vec![];
    let mut image = [[false; 200]; 200];
    let mut image2 = [[false; 200]; 200];
    let mut row = 0;
    for l in read_to_string("input20.txt")?.lines() {
        if l == "" {
            first = false;
            continue;
        }
        if first {
            expansion.extend(l.chars().map(|x| x == '#'));
        } else {
            for (col, c) in l.chars().enumerate() {
                image[row + 20][col + 20] = c == '#';
            }
            row += 1;
        }
    }

    println!(
        "{}",
        image[10..190]
            .iter()
            .map(|row| row[10..190]
                .iter()
                .map(|x| if *x { '#' } else { '.' })
                .join(""))
            .join("\n"),
    );

    for k in 0..2 {
        dbg!(k);
        for row in 1..image.len() as isize - 1 {
            for col in 1..image[0].len() as isize - 1 {
                let mut bits = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        bits *= 2;
                        if image[(row + dr) as usize][(col + dc) as usize] {
                            bits += 1;
                        }
                    }
                }
                image2[row as usize][col as usize] = expansion[bits];
            }
        }
        image = image2;
        println!(
            "{}",
            image[10..190]
                .iter()
                .map(|row| row[10..190]
                    .iter()
                    .map(|x| if *x { '#' } else { '.' })
                    .join(""))
                .join("\n"),
        );
    }

    println!(
        "{:?}",
        image[10..190]
            .iter()
            .map(|row| row[10..190].iter().filter(|x| **x).count())
            .sum::<usize>()
    );

    Ok(())
}

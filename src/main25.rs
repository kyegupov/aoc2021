use core::panic;
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

    let mut board: Vec<Vec<char>> = read_to_string("input25.txt")?
        .lines()
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    let mut moved = true;
    let mut count = 0usize;
    while moved {
        moved = false;
        let mut board2 = board.clone();
        for (j, row) in board.iter().enumerate() {
            for (i, c) in row.iter().enumerate() {
                if *c == '>' {
                    let ii = (i + 1) % board[0].len();
                    if board[j][ii] == '.' {
                        board2[j][ii] = '>';
                        board2[j][i] = '.';
                        moved = true;
                    }
                }
            }
        }
        board = board2;
        let mut board2 = board.clone();
        for (j, row) in board.iter().enumerate() {
            for (i, c) in row.iter().enumerate() {
                if *c == 'v' {
                    let jj = (j + 1) % board.len();
                    if board[jj][i] == '.' {
                        board2[jj][i] = 'v';
                        board2[j][i] = '.';
                        moved = true;
                    }
                }
            }
        }
        board = board2;

        count += 1;
        dbg!(count);
        // println!(
        //     "{}",
        //     board
        //         .iter()
        //         .map(|x| x.iter().collect::<String>())
        //         .join("\n")
        // );
        // if count > 2 {
        //     panic!();
        // }
    }

    dbg!(count);

    Ok(())
}

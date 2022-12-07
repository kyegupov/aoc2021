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

type Positions = BTreeMap<(usize, usize), char>;

#[derive(Default)]
struct Memoized(BTreeMap<Positions, (usize, Option<Positions>)>);

fn cost(c: char) -> usize {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!(),
    }
}

fn target(c: char) -> usize {
    match c {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
        _ => panic!(),
    }
}

fn print(p: &Positions, maxy: usize) {
    for y in 0..=maxy {
        for x in 0..=10 {
            let c = if (y > 0 && !matches!(x, 2 | 4 | 6 | 8)) {
                '#'
            } else {
                *p.get(&(x, y)).unwrap_or(&'.')
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

impl Memoized {
    fn solve(&mut self, ps: Positions, maxy: usize) -> usize {
        if let Some(s) = self.0.get(&ps) {
            return s.0;
        }
        // dbg!(&ps);
        // print(&ps, maxy);
        // panic!();

        if ps.iter().all(|(&(x, y), &c)| x == target(c) && y > 0) {
            return 0;
        }

        let mut mins = 999999999999usize;
        let mut best_p = None;
        for (&(x, y), &c) in ps.iter().filter(|(&(x, y), &c)| y == 0) {
            let tx = target(c);
            let target_hole_is_ok_to_settle = (1..=maxy)
                .map(|yy| ps.get(&(tx, yy)))
                .all(|cc| cc == Some(&c) || cc == None);
            if target_hole_is_ok_to_settle {
                let ty = (1..=maxy)
                    .filter(|yy| ps.get(&(tx, *yy)) == None)
                    .last()
                    .unwrap();
                let range = if tx > x { x + 1..tx + 1 } else { tx..x };
                if range.clone().all(|xx| !ps.contains_key(&(xx, 0))) {
                    let mut ps2 = ps.clone();
                    ps2.remove(&(x, y));
                    ps2.insert((tx, ty), c);
                    let s = (ty as isize + (tx as isize - x as isize).abs()) as usize * cost(c)
                        + self.solve(ps2.clone(), maxy);
                    if s < mins {
                        mins = s;
                        best_p = Some(ps2);
                    }
                }
            }
        }
        if best_p.is_none() {
            for (&(x, y), &c) in ps.iter().filter(|(&(x, y), &c)| y > 0) {
                if y > 0 {
                    let can_move = (1..y).all(|yy| !ps.contains_key(&(x, yy)));
                    let settled =
                        target(c) == x && (y + 1..=maxy).all(|yy| ps.get(&(x, yy)) == Some(&c));
                    if can_move && !settled {
                        let possible_xs = (x + 1..=10)
                            .take_while(|xx| !ps.contains_key(&(*xx, 0)))
                            .chain((0..x).rev().take_while(|xx| !ps.contains_key(&(*xx, 0))))
                            .filter(|x| !matches!(x, 2 | 4 | 6 | 8))
                            .collect_vec();
                        for xx in possible_xs {
                            let mut ps2 = ps.clone();
                            ps2.remove(&(x, y));
                            ps2.insert((xx, 0), c);
                            let s = (y as isize + (x as isize - xx as isize).abs()) as usize
                                * cost(c)
                                + self.solve(ps2.clone(), maxy);
                            if s < mins {
                                mins = s;
                                best_p = Some(ps2);
                            }
                        }
                    }
                }
            }
        }

        self.0.insert(ps, (mins, best_p.clone()));
        mins
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");

    let mut lines: Vec<String> = read_to_string("input23.txt")?
        .lines()
        .map(|x| x.to_owned())
        .collect_vec();

    if part2 {
        lines.insert(3, "  #D#B#A#C#".to_owned());
        lines.insert(3, "  #D#C#B#A#".to_owned());
    }

    let mut positions = BTreeMap::new();

    for y in 1..lines.len() - 1 {
        for (x, c) in lines[y + 1].chars().enumerate() {
            if c >= 'A' && c <= 'D' {
                positions.insert((x - 1, y), c);
            }
        }
    }

    let mut m = Memoized::default();
    let s = m.solve(positions.clone(), if part2 { 4 } else { 2 });

    // let mut p = positions.clone();
    // let mut ss2 = s as isize;
    // while m.0.contains_key(&p) {
    //     let (ss, pp) = &m.0[&p];
    //     println!("{}", ss2 - *ss as isize);
    //     print(&p);
    //     ss2 = *ss as isize;
    //     p = pp.as_ref().unwrap().clone();
    // }

    dbg!(s);

    Ok(())
}

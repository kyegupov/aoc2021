use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::digit0;
use nom::combinator::map;
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
use std::error::Error;
use std::fmt;
use std::fs::read_to_string;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct State {
    pos1: u64,
    pos2: u64,
    s1: u64,
    s2: u64,
}

#[derive(Default, Debug)]
struct Memoized(BTreeMap<State, (u64, u64)>);

fn wrap(p: u64) -> u64 {
    ((p - 1) % 10) + 1
}

impl Memoized {
    fn solve(&mut self, st: State) -> (u64, u64) {
        if let Some(memoized) = self.0.get(&st) {
            return *memoized;
        }

        let mut wins1 = 0u64;
        let mut wins2 = 0u64;
        for i in (0..3)
            .map(|_| 1..=3)
            .multi_cartesian_product()
            .map(|x| x.iter().sum::<u64>())
        {
            let pos1 = wrap(st.pos1 + i);
            if st.s1 + pos1 >= 21 {
                wins1 += 1;
            } else {
                for j in (0..3)
                    .map(|_| 1..=3)
                    .multi_cartesian_product()
                    .map(|x| x.iter().sum::<u64>())
                {
                    let pos2 = wrap(st.pos2 + j);
                    if st.s2 + pos2 >= 21 {
                        wins2 += 1;
                    } else {
                        let (w1, w2) = self.solve(State {
                            pos1,
                            pos2,
                            s1: st.s1 + pos1,
                            s2: st.s2 + pos2,
                        });
                        wins1 += w1;
                        wins2 += w2;
                    }
                }
            }
        }
        self.0.insert(st, (wins1, wins2));
        (wins1, wins2)
    }
}

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

    let mut mem = Memoized::default();
    let (w1, w2) = mem.solve(State {
        pos1,
        pos2,
        s1: 0,
        s2: 0,
    });
    // dbg!(&mem);
    dbg!(max(w1, w2));

    Ok(())
}

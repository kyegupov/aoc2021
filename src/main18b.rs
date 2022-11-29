use itertools::Itertools as _;
use nom::branch::alt;
use nom::character::complete::digit0;
use nom::combinator::map;
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
use std::error::Error;
use std::fmt;
use std::fs::read_to_string;

extern crate nom;
use nom::{bytes::complete::tag, IResult};

#[derive(Clone)]
enum SnailNode {
    Single(i64),
    Pair(Box<SnailNum>),
}

#[derive(Clone)]
struct SnailNum(SnailNode, SnailNode);

impl fmt::Debug for SnailNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnailNode::Single(x) => write!(f, "{}", x),
            SnailNode::Pair(p) => write!(f, "{:?}", p),
        }
    }
}
impl fmt::Debug for SnailNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}, {:?}]", self.0, self.1)
    }
}

fn snail_node_parse(input: &str) -> IResult<&str, SnailNode> {
    alt((
        map(snail_num_parse, |pair| SnailNode::Pair(Box::new(pair))),
        map(digit0, |x: &str| SnailNode::Single(x.parse().unwrap())),
    ))(input)
}

fn snail_num_parse(input: &str) -> IResult<&str, SnailNum> {
    let (input, _) = tag("[")(input)?;
    let (input, first) = snail_node_parse(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, second) = snail_node_parse(input)?;
    let (input, _) = tag("]")(input)?;

    Ok((input, SnailNum(first, second)))
}

fn add(x: SnailNum, y: SnailNum) -> SnailNum {
    let mut as_node = SnailNode::Pair(Box::new(SnailNum(
        SnailNode::Pair(Box::new(x)),
        SnailNode::Pair(Box::new(y)),
    )));
    loop {
        let exploded = maybe_explode(&mut as_node, 0, None, None);
        if !exploded {
            let split = maybe_split(&mut as_node);
            if !split {
                break;
            }
        };
    }
    match as_node {
        SnailNode::Single(_) => panic!(),
        SnailNode::Pair(p) => *p,
    }
}

fn maybe_explode(
    n: &mut SnailNode,
    nesting: usize,
    prev_scalar: Option<&mut i64>,
    next_scalar: Option<&mut i64>,
) -> bool {
    match n {
        SnailNode::Pair(p) => {
            if nesting == 4 {
                match (&p.0, &p.1) {
                    (SnailNode::Single(x), SnailNode::Single(y)) => {
                        prev_scalar.map(|a| *a += x);
                        next_scalar.map(|a| *a += y);
                        *n = SnailNode::Single(0);
                        return true;
                    }
                    _ => {
                        panic!()
                    }
                }
            } else {
                if maybe_explode(
                    &mut p.0,
                    nesting + 1,
                    prev_scalar,
                    get_first_scalar_ptr(&mut p.1),
                ) {
                    return true;
                };
                if maybe_explode(
                    &mut p.1,
                    nesting + 1,
                    get_last_scalar_ptr(&mut p.0),
                    next_scalar,
                ) {
                    return true;
                };
            }
        }
        _ => {}
    }
    return false;
}

fn maybe_split(n: &mut SnailNode) -> bool {
    match n {
        SnailNode::Pair(p) => {
            if maybe_split(&mut p.0) {
                return true;
            };
            if maybe_split(&mut p.1) {
                return true;
            };
        }
        SnailNode::Single(x) => {
            if *x >= 10 {
                *n = SnailNode::Pair(Box::new(SnailNum(
                    SnailNode::Single((*x) / 2),
                    SnailNode::Single((*x + 1) / 2),
                )));
                return true;
            }
        }
    }
    return false;
}

fn get_first_scalar_ptr(p: &mut SnailNode) -> Option<&mut i64> {
    match p {
        SnailNode::Single(x) => Some(x),
        SnailNode::Pair(x) => get_first_scalar_ptr(&mut x.0),
    }
}

fn get_last_scalar_ptr(p: &mut SnailNode) -> Option<&mut i64> {
    match p {
        SnailNode::Single(x) => Some(x),
        SnailNode::Pair(x) => get_last_scalar_ptr(&mut x.1),
    }
}

fn magnitude(p: &SnailNode) -> i64 {
    match p {
        SnailNode::Single(x) => *x,
        SnailNode::Pair(x) => 3 * magnitude(&x.0) + 2 * magnitude(&x.1),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut nums: VecDeque<SnailNum> = read_to_string("input18.txt")?
        .lines()
        .map(|x| snail_num_parse(x).unwrap().1)
        .collect();

    println!("{:?}", nums);

    let mut mx = 0;
    for (i, n) in nums.iter().enumerate() {
        for (j, n2) in nums.iter().enumerate() {
            if i != j {
                let m = magnitude(&SnailNode::Pair(Box::new(add(n.clone(), n2.clone()))));
                if m > mx {
                    mx = m;
                }
            }
        }
    }

    println!("{:?}", mx);

    Ok(())
}

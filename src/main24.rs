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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct State {
    x: i64,
    y: i64,
    z: i64,
}
// [step][z] -> (initial_z, input)
type Memoized = BTreeMap<usize, BTreeMap<i64, (i64, i64)>>;

#[derive(Clone, Copy, Debug)]
enum Operand {
    Reg(char),
    Num(i64),
}
#[derive(Debug)]
struct Command {
    op: Op,
    op1: char,
    op2: Operand,
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

fn run(
    lines: &Vec<Command>,
    input_steps: &Vec<usize>,
    memoized: &mut Memoized,
    step: usize,
    overwrite: bool,
) {
    let source_zs = if step > 0 {
        memoized[&(step - 1)].keys().cloned().collect_vec()
    } else {
        vec![0]
    };
    for src_z in source_zs {
        for w in 1..10 {
            let mut state = State {
                x: 0,
                y: 0,
                z: src_z,
            };
            for cmd in &lines[1 + input_steps[step]..] {
                match cmd.op {
                    Op::Inp => {
                        break;
                    }
                    Op::Add => {
                        *reg(&mut state, cmd.op1) =
                            val(&state, w, Operand::Reg(cmd.op1)) + val(&state, w, cmd.op2)
                    }
                    Op::Mul => {
                        *reg(&mut state, cmd.op1) =
                            val(&state, w, Operand::Reg(cmd.op1)) * val(&state, w, cmd.op2)
                    }
                    Op::Div => {
                        *reg(&mut state, cmd.op1) =
                            val(&state, w, Operand::Reg(cmd.op1)) / val(&state, w, cmd.op2)
                    }
                    Op::Mod => {
                        *reg(&mut state, cmd.op1) =
                            val(&state, w, Operand::Reg(cmd.op1)) % val(&state, w, cmd.op2)
                    }
                    Op::Eql => {
                        *reg(&mut state, cmd.op1) =
                            if { val(&state, w, Operand::Reg(cmd.op1)) == val(&state, w, cmd.op2) }
                            {
                                1
                            } else {
                                0
                            }
                    }
                }
            }
            let sub_map = memoized.entry(step).or_default();

            if overwrite {
                sub_map.insert(state.z, (src_z, w));
            } else {
                sub_map.entry(state.z).or_insert((src_z, w));
            }
        }
    }
}

fn val(state: &State, w: i64, operand: Operand) -> i64 {
    match operand {
        Operand::Reg('x') => state.x,
        Operand::Reg('y') => state.y,
        Operand::Reg('z') => state.z,
        Operand::Reg('w') => w,
        Operand::Num(x) => x,
        _ => panic!(),
    }
}

fn reg(state: &mut State, name: char) -> &mut i64 {
    match name {
        'x' => &mut state.x,
        'y' => &mut state.y,
        'z' => &mut state.z,
        _ => panic!("{}", name),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let part2 = env::args().any(|x| x == "2");

    let mut memoized: Memoized = BTreeMap::new();

    let lines: Vec<String> = read_to_string("input24.txt")?
        .lines()
        .map(|x| x.to_owned())
        .collect_vec();
    let program = lines
        .into_iter()
        .map(|l| {
            let words = l.split(' ').collect_vec();
            Command {
                op: match words[0] {
                    "inp" => Op::Inp,
                    "add" => Op::Add,
                    "mul" => Op::Mul,
                    "eql" => Op::Eql,
                    "div" => Op::Div,
                    "mod" => Op::Mod,
                    _ => panic!(),
                },
                op1: words[1].chars().next().unwrap(),
                op2: if words.len() > 2 {
                    match words[2] {
                        "x" | "y" | "z" | "w" => Operand::Reg(words[2].chars().next().unwrap()),
                        _ => Operand::Num(words[2].parse().unwrap()),
                    }
                } else {
                    Operand::Num(0)
                },
            }
        })
        .collect_vec();
    let input_steps = program
        .iter()
        .enumerate()
        .filter(|(_, x)| x.op == Op::Inp)
        .map(|(i, _)| i)
        .collect_vec();

    for i in 0..input_steps.len() {
        run(&program, &input_steps, &mut memoized, i, !part2);
        dbg!(i, memoized.values().map(|x| x.keys().len()).sum::<usize>());
    }

    let mut digits = vec![];
    let mut z = 0;
    for i in (0..input_steps.len()).rev() {
        let (zz, d) = memoized[&i][&z];
        digits.push(d);
        z = zz;
    }
    dbg!(digits.iter().rev().map(|x| x.to_string()).join(""));

    Ok(())
}

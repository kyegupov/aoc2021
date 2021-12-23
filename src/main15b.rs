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

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input15.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let dims = ((lines[0].chars().count()) as isize, (lines.len()) as isize);

    let mut a: BTreeMap<(isize, isize), i64> = BTreeMap::new();
    for yy in 0..5 {
        for xx in 0..5 {
            for (y, l) in lines.iter().enumerate() {
                for (x, c) in l.chars().enumerate() {
                    a.insert(
                        (x as isize + xx * dims.0, y as isize + yy * dims.1),
                        ((c.to_string().parse::<i64>().unwrap() + (yy as i64) + (xx as i64) - 1)
                            % 9)
                            + 1,
                    );
                }
            }
        }
    }

    let mut unvisited: BinaryHeap<Cell> = BinaryHeap::new();
    unvisited.push(Cell {
        xy: (0, 0),
        dist: 0,
    });
    let mut dist: BTreeMap<(isize, isize), i64> = BTreeMap::new();
    dist.insert((0, 0), 0);

    let target = ((dims.0 * 5 - 1) as isize, (dims.1 * 5 - 1) as isize);

    let res = loop {
        let f = unvisited.pop().unwrap();
        if f.xy == target {
            break f.dist;
        }
        if f.dist > dist[&f.xy] {
            continue;
        }
        for dx in -1isize..=1 {
            for dy in -1isize..=1 {
                if (dx != 0) ^ (dy != 0) {
                    let xy2 = (f.xy.0 + dx, f.xy.1 + dy);
                    if !a.contains_key(&xy2) {
                        continue;
                    }
                    let next = Cell {
                        dist: f.dist + a[&xy2],
                        xy: xy2,
                    };
                    if next.dist < *dist.get(&xy2).unwrap_or(&9999999) {
                        unvisited.push(next);
                        dist.insert(xy2, next.dist);
                    }
                }
            }
        }
        // if f.xy == (1, 2) {
        //     break 0;
        // }
    };

    for y in 0..10 {
        for x in 0..30 {
            print!("{:2}", a.get(&(x, y)).unwrap_or(&99));
        }
        println!();
    }

    println!("{}", res);
    Ok(())
}

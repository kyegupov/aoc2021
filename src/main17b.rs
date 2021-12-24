use itertools::Itertools as _;
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList};
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    // let target = ((20, 30), (-10, -5));
    let target = ((56, 76), (-162, -134));
    let mut best = (0, 0);
    let mut highest = 0;

    let mut ans = BTreeSet::new();

    for dx0 in 0..77 {
        for dy0 in -162..500 {
            let mut dx = dx0;
            let mut dy = dy0;
            let mut x = 0;
            let mut y = 0;
            let mut run_highest = 0;
            while x <= target.0 .1 && y >= target.1 .0 {
                x += dx;
                y += dy;
                if y > run_highest {
                    run_highest = y;
                }
                if dx > 0 {
                    dx -= 1;
                }
                dy -= 1;
                if x >= target.0 .0 && x <= target.0 .1 && y >= target.1 .0 && y <= target.1 .1 {
                    ans.insert((dx0, dy0));
                    if run_highest > highest {
                        highest = run_highest;
                        best = (dx0, dy0);
                        dbg!(highest, best);
                    }
                    break;
                }
            }
        }
    }

    dbg!(ans.len());
    Ok(())
}

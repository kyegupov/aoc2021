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

fn rotate(coords: Coords, orientation: Orientation) -> Coords {
    let as_slice = [coords.0, coords.1, coords.2];
    let (axes, signs) = orientation;
    (
        as_slice[axes.0] * signs.0,
        as_slice[axes.1] * signs.1,
        as_slice[axes.2] * signs.2,
    )
}

fn sub(coords: Coords, diff: Coords) -> Coords {
    (coords.0 - diff.0, coords.1 - diff.1, coords.2 - diff.2)
}

fn add(coords: Coords, diff: Coords) -> Coords {
    (coords.0 + diff.0, coords.1 + diff.1, coords.2 + diff.2)
}

extern crate nom;
fn main() -> Result<(), Box<dyn Error>> {
    let mut scanners: Vec<Vec<Coords>> = vec![];
    let mut scanner: Vec<Coords> = vec![];
    for l in read_to_string("input19.txt")?.lines() {
        if l.starts_with("---") {
        } else if l == "" {
            scanners.push(scanner);
            scanner = vec![];
        } else {
            scanner.push(
                l.split(',')
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
            );
        }
    }
    scanners.push(scanner);
    let mut axes_r = [0, 1, 2];
    let mut axes_l = [0, 2, 1];
    let mut orientations: Vec<Orientation> = vec![];
    for offset in 0..3 {
        for xsign in [1, -1] {
            for ysign in [1, -1] {
                orientations.push((
                    axes_r.iter().cloned().collect_tuple().unwrap(),
                    (xsign, ysign, xsign * ysign),
                ));
                orientations.push((
                    axes_l.iter().cloned().collect_tuple().unwrap(),
                    (xsign, ysign, -xsign * ysign),
                ));
            }
        }
        axes_l.rotate_left(1);
        axes_r.rotate_left(1);
    }

    let mut scanner_offsets_in_orientations: BTreeMap<(usize, usize), Vec<BTreeSet<Coords>>> =
        BTreeMap::new();

    for (i1, s1) in scanners.iter().enumerate() {
        for (o2, ori) in orientations.iter().enumerate() {
            let mut all_offsets = vec![];
            for (ib1, b1) in s1.iter().enumerate() {
                let mut offsets = BTreeSet::new();
                for b2 in s1 {
                    if b1 == b2 {
                        continue;
                    }
                    let rotated = rotate((b2.0 - b1.0, b2.1 - b1.1, b2.2 - b1.2), *ori);
                    if (i1 == 1) && (o2 == 4) && b2.0 - b1.0 == -140 {
                        dbg!((b2.0 - b1.0, b2.1 - b1.1, b2.2 - b1.2));
                        dbg!(ib1, rotated);
                    }

                    offsets.insert(rotated);
                }
                all_offsets.push(offsets);
            }
            scanner_offsets_in_orientations.insert((i1, o2), all_offsets);
        }
    }
    let mut processed_scanners = BTreeSet::new();
    processed_scanners.insert(0);
    let mut unprocessed_scanners = BTreeSet::new();
    for s in 1..scanners.len() {
        unprocessed_scanners.insert(s);
    }
    let mut conversions: BTreeMap<usize, (usize, Orientation, Coords)> = BTreeMap::new();

    while !unprocessed_scanners.is_empty() {
        for &i2 in unprocessed_scanners.iter() {
            'outer: for &i1 in processed_scanners.iter() {
                // println!("checking {} {}", i1, i2);
                let s1 = &scanners[i1];
                let s2 = &scanners[i2];
                for b1 in 0..s1.len() {
                    for b2 in 0..s2.len() {
                        for (o2, orientation) in orientations.iter().enumerate() {
                            let similar = scanner_offsets_in_orientations[&(i1, 0)][b1]
                                .intersection(&scanner_offsets_in_orientations[&(i2, o2)][b2])
                                .collect_vec();
                            if similar.len() >= 11 {
                                // println!(
                                //     "overlap between scanners {} {} {:?}",
                                //     i1, i2, orientation
                                // );
                                let pos1 = s1[b1];
                                let rotated = rotate(s2[b2], *orientation);
                                let offs = sub(pos1, rotated);
                                conversions.insert((i2), (i1, *orientation, offs));

                                // let all_rotated: BTreeSet<_> = s2
                                //     .iter()
                                //     .cloned()
                                //     .map(|x| add(rotate(x, *orientation), offs))
                                //     .collect();
                                // let s1_set: BTreeSet<_> = s1.iter().cloned().collect();
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
        for m in conversions.keys() {
            unprocessed_scanners.remove(m);
            processed_scanners.insert(*m);
        }
    }

    let mut beacons_in_s0_space = BTreeSet::new();
    for s in &scanners[0] {
        beacons_in_s0_space.insert(s.clone());
    }
    for (i1, s1) in scanners.iter().enumerate() {
        // dbg!(pos);
        for &pos1 in s1 {
            let mut in_space = i1;
            let mut pos = pos1;
            while in_space != 0 {
                // dbg!(in_space);
                let (is2, o, offs) = conversions[&in_space];
                in_space = is2;
                pos = rotate(pos, o);
                pos = add(pos, offs);
            }
            // dbg!(pos);
            beacons_in_s0_space.insert(pos);
        }
    }

    println!("{:?}", beacons_in_s0_space.len());

    Ok(())
}

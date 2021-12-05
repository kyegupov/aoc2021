use std::collections::BTreeSet;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input04.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let numbers: Vec<i64> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    let mut boards: Vec<Vec<Vec<(i64, bool)>>> = vec![vec![]];
    for l in &lines[1..] {
        if l.trim() == "" {
            boards.push(vec![]);
        } else {
            let row: Vec<(i64, bool)> = l
                .split_whitespace()
                .map(|x| (x.parse().unwrap(), false))
                .collect();
            boards.last_mut().unwrap().push(row);
        }
    }
    if boards.last().unwrap().is_empty() {
        boards.pop();
    }

    let mut res = 0i64;
    let mut won = BTreeSet::new();

    'out: for n in numbers {
        for (i, b) in boards.iter_mut().enumerate() {
            if won.contains(&i) {
                continue;
            }
            for row in b.iter_mut() {
                for e in row.iter_mut() {
                    if n == e.0 {
                        e.1 = true;
                    }
                }
            }
            // dbg!(&i, &b);
            if b.iter().any(|r| r.iter().all(|e| e.1)) || (0..5).any(|i| b.iter().all(|x| x[i].1)) {
                res = b
                    .iter()
                    .map(|r| {
                        r.iter()
                            .filter_map(|e| if !e.1 { Some(e.0) } else { None })
                            .sum::<i64>()
                    })
                    .sum::<i64>()
                    * n;
                won.insert(i);
            }
        }
    }

    println!("{}", res);
    Ok(())
}

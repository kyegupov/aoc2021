use std::collections::BTreeSet;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input04.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let numbers: Vec<i64> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    let mut boards: Vec<Vec<BTreeSet<i64>>> = vec![vec![]];
    for l in &lines[1..] {
        if l.trim() == "" {
            boards.push(vec![]);
        } else {
            let row: BTreeSet<i64> = l.split_whitespace().map(|x| x.parse().unwrap()).collect();
            boards.last_mut().unwrap().push(row);
        }
    }
    if boards.last().unwrap().is_empty() {
        boards.pop();
    }

    let mut res = 0i64;
    dbg!(&boards);

    'out: for n in numbers {
        for b in boards.iter_mut() {
            for row in b.iter_mut() {
                row.remove(&n);
            }
            if b.iter().any(|x| x.is_empty()) {
                res = b.iter().map(|r| r.iter().sum::<i64>()).sum::<i64>() * n;
                break 'out;
            }
            // dbg!(&b);
        }
    }

    println!("{}", res);
    Ok(())
}

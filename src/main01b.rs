use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let x: Vec<i64> = read_to_string("input01.txt")?
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut w = x.windows(3).map(|w| w.iter().sum());
    let mut p = w.next().unwrap();
    let res = w.fold(0, |a, x: i64| {
        let d = if x > p { 1 } else { 0 };
        p = x;
        a + d
    });

    println!("{}", res);
    Ok(())
}

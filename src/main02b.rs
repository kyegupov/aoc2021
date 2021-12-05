use itertools::Itertools as _;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input02.txt")?
        .lines()
        .map(|x| x.into())
        .collect();
    let mut x = 0i64;
    let mut y = 0i64;
    let mut aim = 0i64;
    for l in lines {
        let parts: Vec<&str> = l.split(" ").collect();
        let cmd = parts[0];
        let n: i64 = parts[1].parse().unwrap();
        match cmd {
            "forward" => {
                x += n;
                y += aim * n;
            }
            "backward" => {
                x -= n;
                y += aim * n;
            }
            "up" => {
                aim += n;
            }
            "down" => {
                aim -= n;
            }
            _ => unreachable!(),
        }
    }
    dbg!(x, y);
    let res = x * (-y);

    println!("{}", res);
    Ok(())
}

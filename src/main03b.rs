use itertools::Itertools as _;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string("input03.txt")?
        .lines()
        .map(|x| x.into())
        .collect();

    let mut ox = String::new();
    let mut co2 = String::new();

    for i in 0..lines[0].len() {
        let mut oxb = (0usize, 0usize);
        let mut co2b = (0usize, 0usize);
        let mut co2only = &lines[0];
        for l in &lines {
            let c = l.chars().skip(i).next().unwrap();
            if l.starts_with(&ox) {
                if c == '1' {
                    oxb.1 += 1;
                } else {
                    oxb.0 += 1;
                }
            }
            if l.starts_with(&co2) {
                co2only = l;
                if c == '1' {
                    co2b.1 += 1;
                } else {
                    co2b.0 += 1;
                }
            }
        }
        ox.push(if oxb.1 >= oxb.0 { '1' } else { '0' });
        if co2b.0 + co2b.1 > 1 {
            co2.push(if co2b.1 < co2b.0 { '1' } else { '0' });
        } else if co2b.0 + co2b.1 == 1 {
            co2 = co2only.clone();
        }
        dbg!(&ox, &co2);
    }
    let oxn = i64::from_str_radix(&ox, 2).unwrap();
    let co2n = i64::from_str_radix(&co2, 2).unwrap();

    let res = oxn * co2n;

    println!("{}", res);
    Ok(())
}

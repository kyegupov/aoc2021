use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let mut p = 0;
    let mut c = 0;
    for (i, line) in read_to_string("input01.txt")?.lines().enumerate() {
        let x: i64 = line.parse()?;
        if i > 0 && x > p {
            c += 1;
        }
        p = x;
    }
    println!("{}", c);
    Ok(())
}

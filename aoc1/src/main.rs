use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};

fn read_input(filename: &str) -> Result<Vec<u64>, Error> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() {
    let input = read_input("input").unwrap();
    let part1: u64 = input.iter().map(|fuel| fuel / 3 - 2).sum();
    let part2: u64 = input
        .iter()
        .map(|fuel| {
            let mut tot = 0;
            let mut f: u64 = *fuel;
            loop {
                if f / 3 <= 2 {
                    break;
                }
                f = f / 3 - 2;
                tot += f;
            }
            tot
        })
        .sum();
    println!("Part 1: {}", part1);
    println!("Part2: {}", part2);
}

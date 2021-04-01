use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

use intcode::*;

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.unwrap())
        .next()
        .unwrap()
        .split(',')
        .map(|x| {
            x.parse::<i64>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))
        })
        .collect()
}

fn main() {
    let filename = "5.in";
    let file = File::open(filename).unwrap();
    let mut instructions = match read(file) {
        Ok(x) => x,
        Err(e) => panic!("Error reading file: {}", e),
    };

    let mut input = Instr {
        instr: instructions.clone(),
        input: vec![1],
        output: vec![],
        relative_base: 0,
    };
    input.iterate();
    let out1 = input.output.pop().expect("Error: No output");
    println!("Part 1: {}", out1);
    input.instr = instructions.clone();
    input.input = vec![5];
    input.iterate();
    let out2 = input.output.pop().expect("Error Part 2: No Output");
    println!("Part 2: {}", out2);
}

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::time::Instant;

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
    let filename = "2.in";
    let file = File::open(filename).unwrap();
    let mut instructions = match read(file) {
        Ok(x) => x,
        Err(e) => panic!("Error reading file: {}", e),
    };

    // Replace Position 1 with 12 and Position 2 with 2 according to the instructions
    instructions[1] = 12;
    instructions[2] = 2;
    let start = Instant::now();
    let mut input = Instr {
        instr: instructions.clone(),
        input: vec![],
        output: vec![],
        relative_base: 0,
    };
    input.iterate();
    println!("Part 1: {}", input.instr[0]);

    // Part 2: Find noun and verb that produces 19690720 as output
    let mut flag = false;
    for i in 0..100 {
        for j in 0..100 {
            input.instr = instructions.clone();
            input.instr[1] = i;
            input.instr[2] = j;
            input.iterate();
            match input.instr[0] == 19690720 {
                true => {
                    println!("Part 2: {}", 100 * i + j);
                    flag = true;
                    break;
                }
                false => (),
            }
        }
        if flag {
            break;
        }
    }
    let duration = start.elapsed();
    println!("Code ran in {:?}", duration);
}

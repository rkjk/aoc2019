use std::cmp::max;
use std::collections::HashSet;
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

fn swap(arr: &mut Vec<i64>, i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn perm(k: usize, arr: &mut Vec<i64>, permutation: &mut HashSet<Vec<i64>>) {
    if k == 1 {
        permutation.insert(arr.to_vec());
    } else {
        for i in 0..k {
            perm(k - 1, arr, permutation);
            if i < k - 1 {
                if k % 2 == 0 {
                    swap(arr, i, k - 1);
                } else {
                    swap(arr, 0, k - 1);
                }
            }
        }
    }
}

fn main() {
    let filename = "7.in";
    let file = File::open(filename).unwrap();
    let mut instructions = match read(file) {
        Ok(x) => x,
        Err(e) => panic!("Error reading file: {}", e),
    };

    let mut permutations: HashSet<Vec<i64>> = HashSet::new();
    perm(5, &mut vec![0, 1, 2, 3, 4], &mut permutations);

    let mut amplifiers: Vec<Instr> = vec![];
    let mut prev_output = 0;
    for _ in 0..5 {
        amplifiers.push(Instr {
            instr: instructions.clone(),
            input: vec![],
            output: vec![],
            relative_base: 0,
            instr_ptr: 0,
        });
    }
    let mut max_output = 0;
    for perm in permutations {
        for (i, phase) in perm.iter().enumerate() {
            amplifiers[i].input = vec![prev_output, *phase];
            amplifiers[i].instr = instructions.clone();
            amplifiers[i].instr_ptr = 0;
            amplifiers[i].iterate(true);
            prev_output = amplifiers[i]
                .output
                .pop()
                .expect("Error in Amplifier: No output");
        }
        max_output = max(max_output, prev_output);
        prev_output = 0;
    }
    println!("Part 1: {}", max_output);

    // Part 2
    permutations = HashSet::new();
    perm(5, &mut vec![5, 6, 7, 8, 9], &mut permutations);
    let mut opcode = 0;
    max_output = 0;
    for perm in permutations {
        amplifiers = Vec::new();
        for i in 0..5 {
            amplifiers.push(Instr {
                instr: instructions.clone(),
                input: vec![],
                output: vec![],
                relative_base: 0,
                instr_ptr: 0,
            });
        }
        opcode = 0;
        prev_output = 0;
        let mut count = 0;
        while opcode != 99 {
            for (i, phase) in perm.iter().enumerate() {
                if amplifiers[i].instr[amplifiers[i].instr_ptr] == 99 {
                    continue;
                }
                match count == 0 {
                    true => {
                        amplifiers[i].input = vec![prev_output, *phase];
                    }
                    false => {
                        amplifiers[i].input = vec![prev_output];
                    }
                };
                amplifiers[i].iterate(true);
                prev_output = amplifiers[i]
                    .output
                    .pop()
                    .expect("Error in Amplifier: No output");
                opcode = amplifiers[i].instr[amplifiers[i].instr_ptr];
            }
            count += 1;
        }
        max_output = max(max_output, prev_output);
    }
    println!("Part 2: {}", max_output);
}

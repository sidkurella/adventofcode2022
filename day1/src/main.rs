use std::{collections::BinaryHeap, io::stdin};

const ELVES: usize = 3;

fn main() {
    let mut current_elf: usize = 0;
    let mut elves = BinaryHeap::<usize>::new();
    for line in stdin().lines() {
        let raw_line = line.unwrap();
        let l = raw_line.trim();
        if l.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            let cals: Result<usize, std::num::ParseIntError> = l.parse();
            if cals.is_ok() {
                current_elf += cals.unwrap();
            } else {
                eprintln!("Failed to parse {} (error: {:?})", l, cals)
            }
        }
    }

    let mut sum = 0;
    for _ in 0..ELVES {
        let val = elves.pop().unwrap();
        println!("{}", val);
        sum += val;
    }
    println!("{}", sum);
}

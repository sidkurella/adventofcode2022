use std::{collections::VecDeque, io::stdin};
use text_io::scan;

fn main() {
    let lines: Vec<String> = stdin().lines().map(|x| x.unwrap()).collect();
    let mut iter = lines.iter();
    let mut line = iter.next().unwrap();

    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    // Parse initial state.
    loop {
        if line.starts_with(" 1 ") {
            // Number of stacks line.
            break;
        }

        let chars: Vec<char> = line.chars().collect();
        let crates = chars.chunks(4);
        for (i, v) in crates.enumerate() {
            let crate_letter = v[1];
            if crate_letter.is_whitespace() {
                continue;
            }

            let stack = match stacks.get_mut(i) {
                Some(s) => s,
                None => {
                    while stacks.len() <= i {
                        stacks.push(VecDeque::new());
                    }
                    stacks.get_mut(i).unwrap()
                }
            };

            stack.push_back(crate_letter)
        }

        line = iter.next().unwrap();
    }

    let num_stacks: usize = line
        .split_whitespace()
        .next_back()
        .unwrap()
        .parse()
        .unwrap();

    while stacks.len() < num_stacks {
        stacks.push(VecDeque::new());
    }

    println!("Initial state:");
    for stack in stacks.iter() {
        println!("{:?}", stack);
    }

    while !line.starts_with("move") {
        line = iter.next().unwrap();
    }

    loop {
        println!("{}", line);

        let n: usize;
        let raw_start: usize;
        let raw_end: usize;
        scan!(line.bytes() => "move {} from {} to {}", n, raw_start, raw_end);

        let start = raw_start - 1;
        let end = raw_end - 1;

        let mut crates = Vec::new();
        for _ in 0..n {
            let c = stacks[start].pop_front().unwrap();
            crates.push(c);
        }
        for c in crates.iter().rev() {
            stacks[end].push_front(*c);
        }

        let l = iter.next();
        if l.is_none() {
            break;
        }
        line = l.unwrap();
    }

    println!("Final state:");
    for stack in stacks.iter() {
        println!("{:?}", stack);
    }
}

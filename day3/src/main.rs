use std::collections::HashSet;
use std::io::stdin;

const GROUP_SIZE: usize = 3;

fn get_priority(c: char) -> Option<usize> {
    if c.is_ascii_lowercase() {
        return Some((c as usize) - ('a' as usize) + 1);
    } else if c.is_ascii_uppercase() {
        return Some((c as usize) - ('A' as usize) + 27);
    } else {
        return None;
    }
}

fn main() {
    let mut sum: usize = 0;
    let lines: Vec<String> = stdin().lines().map(|x| x.unwrap()).collect();
    for groups in lines.as_slice().chunks_exact(GROUP_SIZE) {
        let mut rucksacks = Vec::new();
        for group in groups {
            let l = group.trim();

            let items: HashSet<char> = l.chars().collect();
            rucksacks.push(items);
        }

        let intersection = rucksacks
            .iter()
            .cloned()
            .reduce(|a, x| x.intersection(&a).copied().collect())
            .unwrap();

        let common = intersection.iter().next().unwrap();

        sum += get_priority(*common).unwrap();
    }

    println!("{}", sum);
}

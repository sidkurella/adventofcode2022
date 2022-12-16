use std::collections::HashSet;
use std::io::stdin;

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
    for line in stdin().lines() {
        let raw_line = line.unwrap();
        let l = raw_line.trim();
        let partition = l.len() / 2;
        let (compartment1, compartment2) = (&l[..partition], &l[partition..]);
        let mut items = HashSet::new();
        for c in compartment1.chars() {
            items.insert(c);
        }
        for c in compartment2.chars() {
            if items.contains(&c) {
                sum += get_priority(c).unwrap();
                break;
            }
        }
    }

    println!("{}", sum);
}

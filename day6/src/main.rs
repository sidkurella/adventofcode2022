use std::collections::HashSet;
use std::hash::Hash;
use std::io::stdin;

const WINDOW_SIZE: usize = 4;

fn is_distinct<T: Eq + Hash>(s: &[T]) -> bool {
    let mut set = HashSet::new();
    for e in s {
        if set.contains(e) {
            return false;
        }
        set.insert(e);
    }
    return true;
}

fn main() {
    let mut chars: usize = 4;
    for line in stdin().lines() {
        let raw_line = line.unwrap();
        let l = raw_line.trim();
        let char_vec = l.chars().collect::<Vec<char>>();
        let windows = char_vec.windows(WINDOW_SIZE);
        for window in windows {
            if is_distinct(window) {
                println!("Found window: {:?}", window);
                break;
            }
            chars += 1;
        }
    }

    println!("{}", chars)
}

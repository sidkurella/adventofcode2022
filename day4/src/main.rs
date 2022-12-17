use std::cmp::{max, min};
use std::io::stdin;

#[derive(Copy, Clone, Debug)]
struct Range {
    lower: usize,
    upper: usize,
}

impl Range {
    fn new(lower: usize, upper: usize) -> Range {
        Range { lower, upper }
    }

    fn is_contained(self, other: Range) -> bool {
        other.lower >= self.lower && other.upper <= self.upper
    }

    fn disjoint(self, other: Range) -> bool {
        max(self.lower, other.lower) > min(self.upper, other.upper)
    }

    fn overlaps(self, other: Range) -> bool {
        !self.disjoint(other)
    }

    fn contains(self, other: Range) -> bool {
        return other.is_contained(self);
    }
}

fn main() {
    let mut contained_pairs = 0;
    for line in stdin().lines() {
        let raw_line = line.unwrap();
        let l = raw_line.trim();
        let mut pairs = l.split(',');

        let first = pairs.next().unwrap();
        let mut first_bounds = first.split('-');
        let first_range = Range::new(
            first_bounds.next().unwrap().parse().unwrap(),
            first_bounds.next().unwrap().parse().unwrap(),
        );
        let second = pairs.next().unwrap();
        let mut second_bounds = second.split('-');
        let second_range = Range::new(
            second_bounds.next().unwrap().parse().unwrap(),
            second_bounds.next().unwrap().parse().unwrap(),
        );

        if first_range.overlaps(second_range) {
            contained_pairs += 1;
        }
    }

    println!("{}", contained_pairs);
}

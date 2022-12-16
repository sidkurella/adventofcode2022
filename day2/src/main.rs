use std::io::stdin;

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn points(self) -> usize {
        match self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }

    fn parse_opponent(s: &str) -> Option<Throw> {
        match s {
            "A" => Some(Throw::Rock),
            "B" => Some(Throw::Paper),
            "C" => Some(Throw::Scissors),
            _ => None,
        }
    }

    fn parse_self(s: &str) -> Option<Throw> {
        match s {
            "X" => Some(Throw::Rock),
            "Y" => Some(Throw::Paper),
            "Z" => Some(Throw::Scissors),
            _ => None,
        }
    }

    fn parse_as_outcome(s: &str, opponent: Throw) -> Option<Throw> {
        match s {
            "X" => match opponent {
                Throw::Rock => Some(Throw::Scissors),
                Throw::Paper => Some(Throw::Rock),
                Throw::Scissors => Some(Throw::Paper),
            },
            "Y" => Some(opponent),
            "Z" => match opponent {
                Throw::Rock => Some(Throw::Paper),
                Throw::Paper => Some(Throw::Scissors),
                Throw::Scissors => Some(Throw::Rock),
            },
            _ => None,
        }
    }

    fn score_round(self, opponent: Throw) -> usize {
        let outcome = match (self, opponent) {
            (Throw::Rock, Throw::Rock)
            | (Throw::Paper, Throw::Paper)
            | (Throw::Scissors, Throw::Scissors) => Outcome::Draw,

            (Throw::Rock, Throw::Paper)
            | (Throw::Paper, Throw::Scissors)
            | (Throw::Scissors, Throw::Rock) => Outcome::Loss,

            _ => Outcome::Win,
        };

        return outcome.points() + self.points();
    }
}

fn main() {
    let mut score = 0;
    for line in stdin().lines() {
        let raw_line = line.unwrap();
        let l = raw_line.trim();
        let mut parts = l.split(' ');
        let opponent = parts.next().unwrap();
        let mine = parts.next().unwrap();

        let opponent_throw = Throw::parse_opponent(opponent).unwrap();
        let my_throw = Throw::parse_as_outcome(mine, opponent_throw).unwrap();

        score += my_throw.score_round(opponent_throw);
    }

    println!("{}", score)
}

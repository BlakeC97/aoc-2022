use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum RPSMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn parse_outcome(outcome: &str) -> Option<Self> {
        match outcome {
            "X" => Some(Outcome::Loss),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Win),
            _ => None,
        }
    }
}

impl RPSMove {
    pub fn parse_move(mv: &str) -> Option<Self> {
        match mv {
            "A" => Some(RPSMove::Rock),
            "B" => Some(RPSMove::Paper),
            "C" => Some(RPSMove::Scissors),
            _ => None,
        }
    }

    pub fn value(&self) -> u64 {
        match self {
            RPSMove::Rock => 1,
            RPSMove::Paper => 2,
            RPSMove::Scissors => 3,
        }
    }

    // I can probably be more smart about this, but a lazy cartesian product will do
    pub fn play(&self, other: &RPSMove) -> u64 {
        match (self, other) {
            (RPSMove::Rock, RPSMove::Rock) => 3,
            (RPSMove::Rock, RPSMove::Paper) => 0,
            (RPSMove::Rock, RPSMove::Scissors) => 6,
            (RPSMove::Paper, RPSMove::Rock) => 6,
            (RPSMove::Paper, RPSMove::Paper) => 3,
            (RPSMove::Paper, RPSMove::Scissors) => 0,
            (RPSMove::Scissors, RPSMove::Rock) => 0,
            (RPSMove::Scissors, RPSMove::Paper) => 6,
            (RPSMove::Scissors, RPSMove::Scissors) => 3,
        }
    }

    pub fn determine_move_from_outcome(&self, outcome: &Outcome) -> Self {
        match (self, outcome) {
            (RPSMove::Rock, Outcome::Draw) => RPSMove::Rock,
            (RPSMove::Rock, Outcome::Win) => RPSMove::Paper,
            (RPSMove::Rock, Outcome::Loss) => RPSMove::Scissors,
            (RPSMove::Paper, Outcome::Draw) => RPSMove::Paper,
            (RPSMove::Paper, Outcome::Win) => RPSMove::Scissors,
            (RPSMove::Paper, Outcome::Loss) => RPSMove::Rock,
            (RPSMove::Scissors, Outcome::Draw) => RPSMove::Scissors,
            (RPSMove::Scissors, Outcome::Win) => RPSMove::Rock,
            (RPSMove::Scissors, Outcome::Loss) => RPSMove::Paper,
        }
    }
}

fn main() {
    let input = File::open("data/input2.txt")
        .expect("Failed to load data/input2.txt");
    let reader = BufReader::new(input);

    let mut score = 0u64;
    for line in reader.lines().map_while(Result::ok) {
        let mut raw_moves = line.split_ascii_whitespace();
        let enemy_move = match raw_moves.next() {
            Some(m) => {
                match RPSMove::parse_move(m) {
                    Some(rps_move) => rps_move,
                    None => {
                        eprintln!("Couldn't parse the enemy move! Disqualifying round: '{line}'");
                        continue;
                    }
                }
            }
            None => {
                eprintln!("Couldn't read the enemy move! Disqualifying round: '{line}'");
                continue;
            }
        };
        let outcome = match raw_moves.next() {
            Some(o) => {
                match Outcome::parse_outcome(o) {
                    Some(outcome) => outcome,
                    None => {
                        eprintln!("Couldn't parse the outcome! Disqualifying round: '{line}'");
                        continue;
                    }
                }
            }
            None => {
                eprintln!("Couldn't read the outcome! Disqualifying round: '{line}'");
                continue;
            }
        };

        let friendly_move = enemy_move.determine_move_from_outcome(&outcome);

        score += friendly_move.play(&enemy_move) + friendly_move.value();
    }

    println!("{score}");
}

#[cfg(test)]
mod tests {
    use crate::{Outcome, RPSMove};

    #[test]
    // Following the AoC test case:
    // A Y
    // B X
    // C Z
    fn basic_example() {
        let m1 = RPSMove::parse_move("A").unwrap();
        let o = Outcome::parse_outcome("Y").unwrap();
        let m2 = m1.determine_move_from_outcome(&o);
        assert_eq!(4, m2.play(&m1) + m2.value());

        let m1 = RPSMove::parse_move("B").unwrap();
        let o = Outcome::parse_outcome("X").unwrap();
        let m2 = m1.determine_move_from_outcome(&o);
        assert_eq!(1, m2.play(&m1) + m2.value());

        let m1 = RPSMove::parse_move("C").unwrap();
        let o = Outcome::parse_outcome("Z").unwrap();
        let m2 = m1.determine_move_from_outcome(&o);
        assert_eq!(7, m2.play(&m1) + m2.value());
    }
}
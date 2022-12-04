use std::str::FromStr;

type Score = u32;

#[derive(Eq, PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(&self) -> Score {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn score(&self) -> Score {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Draw => 3,
        }
    }
}

struct Round(Shape, Shape);

impl Round {
    pub fn score(&self) -> Score {
        use Shape::*;
        let my_outcome = match (&self.1, &self.0) {
            (Rock, Scissors) => Outcome::Win,
            (Scissors, Paper) => Outcome::Win,
            (Paper, Rock) => Outcome::Win,
            (Rock, Paper) => Outcome::Loss,
            (Scissors, Rock) => Outcome::Loss,
            (Paper, Scissors) => Outcome::Loss,
            _ => Outcome::Draw,
        };
        self.1.score() + my_outcome.score()
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Opponent: A - Rock, B - Paper, C - Scissors
        // Me: X - Rock, Y - Paper, Z - Scissors
        let opponent_move = match &s[0..1] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => return Err(format!("invalid play {}", &s[0..1])),
        };
        let my_move = match &s[2..3] {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => return Err(format!("invalid opponent play {}", &s[2..3])),
        };
        Ok(Self(opponent_move, my_move))
    }
}

fn main() {
    let total_score: Score = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<Round>().unwrap().score())
        .sum();

    // Part 1 - Total score
    println!("Total score is {}", total_score);

    // Part 2
}

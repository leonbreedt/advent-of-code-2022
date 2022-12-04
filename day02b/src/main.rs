use std::str::FromStr;

type Score = u32;

#[derive(Eq, PartialEq, Debug, Clone)]
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

struct Round(Shape, Outcome);

impl Round {
    pub fn score(&self) -> Score {
        use Shape::*;
        let my_move = match &self.1 {
            Outcome::Win => match self.0 {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
            Outcome::Loss => match self.0 {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
            Outcome::Draw => self.0.clone(),
        };
        self.1.score() + my_move.score()
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opponent_move = match &s[0..1] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => return Err(format!("invalid play {}", &s[0..1])),
        };
        let desired_outcome = match &s[2..3] {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => return Err(format!("invalid desired outcome {}", &s[2..3])),
        };
        Ok(Self(opponent_move, desired_outcome))
    }
}

fn main() {
    let total_score: Score = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<Round>().unwrap().score())
        .sum();

    println!("Total score is {}", total_score);
}

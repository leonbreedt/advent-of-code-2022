use std::str::FromStr;

#[derive(Clone)]
struct Item(char);

impl Item {
    pub fn priority(&self) -> i32 {
        match self.0 {
            'a'..='z' => (self.0 as i32) - 96,
            'A'..='Z' => (self.0 as i32) - 38,
            _ => panic!(),
        }
    }
}

struct Rucksack(Vec<Item>, Vec<Item>);

impl Rucksack {
    fn item_in_both_compartments(&self) -> Option<Item> {
        self.0
            .iter()
            .find(|first| self.1.iter().any(|second| first.0 == second.0))
            .cloned()
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let middle = s.len() / 2;
        let first = s[0..middle].chars().map(Item).collect();
        let second = s[middle..s.len()].chars().map(Item).collect();
        Ok(Self(first, second))
    }
}

fn main() {
    let priority_sum: i32 = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<Rucksack>().unwrap())
        .filter_map(|rucksack| rucksack.item_in_both_compartments())
        .map(|item| item.priority())
        .sum();

    println!(
        "Sum of priorities of item in both rucksacks: {}",
        priority_sum
    );
}

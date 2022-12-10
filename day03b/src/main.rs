use std::{collections::HashSet, fmt::Display, hash::Hash, str::FromStr};

#[derive(Clone, Eq, PartialEq, Hash)]
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

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Rucksack(Vec<Item>, Vec<Item>);

impl Rucksack {
    fn all_items(&self) -> Vec<Item> {
        self.0.iter().chain(self.1.iter()).cloned().collect()
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

fn common_item_across_rucksacks(rucksacks: &[Rucksack]) -> Option<Item> {
    let item_sets: Vec<HashSet<Item>> = rucksacks
        .iter()
        .map(|sack| HashSet::from_iter(sack.all_items()))
        .collect();

    let common_items = item_sets
        .iter()
        .skip(1)
        .fold(item_sets[0].clone(), |acc, set| {
            acc.intersection(set).cloned().collect()
        });

    common_items.iter().next().cloned()
}

fn main() {
    let rucksacks: Vec<Rucksack> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<Rucksack>().unwrap())
        .collect();

    let common_item_priority_sum: i32 = rucksacks
        .chunks(3)
        .filter_map(common_item_across_rucksacks)
        .map(|item| item.priority())
        .sum();

    println!(
        "Sum of the priority of badge items in groups: {:?}",
        common_item_priority_sum
    );
}

use std::str::FromStr;

struct Assignment(u32, u32);

impl Assignment {
    pub fn contains(&self, other: &Assignment) -> bool {
        other.0 <= self.0 && other.1 >= self.1
    }
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut range = s.splitn(2, '-').map(|n| n.parse().unwrap());
        Ok(Self(range.next().unwrap(), range.next().unwrap()))
    }
}

struct AssignmentPair(Assignment, Assignment);

impl FromStr for AssignmentPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        Ok(Self(
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ))
    }
}

fn main() {
    let assignment_pairs: Vec<AssignmentPair> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let fully_contained = assignment_pairs
        .iter()
        .filter(|pair| pair.0.contains(&pair.1) || pair.1.contains(&pair.0))
        .count();

    println!(
        "There are {} assignment pairs fully contained within each other.",
        fully_contained
    );
}

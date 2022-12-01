fn main() {
    let mut elves: Vec<u64> = include_str!("../input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .split(|line| line.is_empty())
        .map(|calories| calories.iter().map(|n| n.parse::<u64>().unwrap()).sum::<u64>())
        .collect();

    elves.sort();

    // part 1: how many calories is the elf carrying the most calories carrying?

    println!(
        "The elf with the most calories is carrying {}.",
        elves.last().unwrap()
    );

    // part 2: how many calories are the top 3 elves carrying in total?

    println!(
        "The three elves carrying the most calories are carrying {}.",
        elves[elves.len() - 3..].iter().sum::<u64>()
    );
}

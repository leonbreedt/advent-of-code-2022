use std::str::FromStr;

struct Crate(char);

struct Stack {
    number: u32,
    crates: Vec<Crate>
}

impl Stack {
    pub fn pop(&mut self) -> Crate {
        self.crates.pop().unwrap()
    }

    fn push(&mut self, c: Crate) {
        self.crates.push(c);
    }
}

fn parse_stacks(s: &str) -> Vec<Stack> {
    let mut lines = s.lines().rev();

    let mut stacks: Vec<Stack> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| Stack { number: n.parse::<u32>().unwrap(), crates: Vec::new() })
        .collect();

    for line in lines {
        let line_chars: Vec<char> = line.chars().collect();
        for (n, crate_chars) in line_chars.chunks(4).enumerate() {
            if crate_chars[1] == ' ' {
                continue;
            }
            stacks[n].crates.push(Crate(crate_chars[1]))
        }
    }

    stacks
}

#[derive(Debug)]
struct Move {
    quantity: u32,
    source_stack: u32,
    target_stack: u32
}

impl Move {
    fn perform(&self, stacks: &mut Vec<Stack>) {
        for _ in 0..self.quantity {
            let crate_to_move = stacks[self.source_stack as usize].pop();
            stacks[self.target_stack as usize].push(crate_to_move);
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        split.next();
        let quantity = split.next().unwrap().parse::<u32>().unwrap();
        split.next();
        let source_stack_idx = split.next().unwrap().parse::<u32>().unwrap() - 1;
        split.next();
        let target_stack_idx = split.next().unwrap().parse::<u32>().unwrap() - 1;

        Ok(Self { quantity, source_stack: source_stack_idx, target_stack: target_stack_idx })
    }
}

fn main() {
    let mut split = include_str!("../input.txt").splitn(2, "\n\n");

    let mut stacks: Vec<Stack> = parse_stacks(split.next().unwrap());
    let moves: Vec<Move> = split.next().unwrap().lines().map(|line| line.parse::<Move>().unwrap()).collect();

    for m in &moves {
        m.perform(&mut stacks);
    }

    for stack in &stacks {
        println!("Stack {}: Crate '{}' is on top.", stack.number, stack.crates.last().unwrap().0);
    }

    print!("Message: ");
    for stack in &stacks {
        print!("{}", stack.crates.last().unwrap().0);
    }
    println!();
}

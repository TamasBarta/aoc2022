use regex::Regex;
use std::{cell::RefCell, error::Error, fmt::Display, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    let mut split = input.split("\n\n");
    let stacks_string = split.next().ok_or(MalformedInput {})?;
    let operations = split
        .next()
        .ok_or(MalformedInput {})?
        .split("\n")
        .filter_map(|operation_str| {
            let regex =
                Regex::new(r"^move (?P<amount>[0-9]+) from (?P<from>[0-9]+) to (?P<to>[0-9]+)$")
                    .ok()?;
            let captures = regex.captures(operation_str)?;

            let amount = captures.name("amount")?.as_str().parse().ok()?;
            let from = captures.name("from")?.as_str().parse().ok()?;
            let to = captures.name("to")?.as_str().parse().ok()?;

            Some(Operation { amount, from, to })
        });

    let mut lines = stacks_string.lines().collect::<Vec<_>>();
    let indexes = lines.pop().ok_or(MalformedInput {})?;
    lines.reverse();

    let stacks: Vec<RefCell<Vec<char>>> = indexes
        .chars()
        .enumerate()
        .filter_map(|(index, column_number)| {
            let _column_number: u8 = column_number.to_string().parse().ok()?;

            let mut new_stack = vec![];
            lines.clone().iter().for_each(|line| {
                if let Some(char) = line.chars().nth(index) {
                    if char != ' ' {
                        new_stack.push(char);
                    }
                }
            });

            Some(RefCell::new(new_stack))
        })
        .collect();

    println!("S: {:?}", stacks);

    for operation in operations {
        println!("S: {:?}", operation);
        let mut tmp_stack = vec![];

        let from: usize = operation.from.into();
        let to: usize = operation.to.into();

        let mut from_stack = stacks
            .iter()
            .nth(from - 1)
            .ok_or(MalformedInput)?
            .borrow_mut();
        let mut to_stack = stacks
            .iter()
            .nth(to - 1)
            .ok_or(MalformedInput)?
            .borrow_mut();

        for _ in 0..operation.amount {
            tmp_stack.push(from_stack.pop().unwrap());
        }

        while let Some(tmp) = tmp_stack.pop() {
            to_stack.push(tmp);
        }
    }
    println!("S: {:?}", stacks);

    let top_items = stacks
        .iter()
        .filter_map(|stack| Some(stack.borrow().last()?.clone().to_string()))
        .collect::<Vec<_>>().join("");

    println!("Top crates: {}", top_items);

    Ok(())
}

#[derive(Debug, Clone)]
struct MalformedInput;

impl Display for MalformedInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "input file too short")
    }
}

impl Error for MalformedInput {}

#[derive(Debug, Clone)]
struct Operation {
    amount: u8,
    from: u8,
    to: u8,
}

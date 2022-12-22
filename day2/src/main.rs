use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    let final_score: u16 = input.split("\n").filter_map(|items| {
        let mut split = items.split(" ");
        let enemy = Item::from_enemy_code(split.next()?);
        let outcome = Outcome::from_outcome_code(split.next()?);

        Some(outcome.score() + enemy.which_gives_outcome(&outcome).score())
    })
    .sum();

    println!("Strategy score: {}", final_score);

    Ok(())
}

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

impl Item {
    fn from_enemy_code(char: impl Into<String>) -> Self {
        match char.into().as_str() {
            "A" => Self::Rock,
            "B" => Self::Paper,
            _ => Self::Scissors,
        }
    }
    fn from_player_code(char: impl Into<String>) -> Self {
        match char.into().as_str() {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            _ => Self::Scissors,
        }
    }

    fn which_gives_outcome(&self, outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Win => match self {
                Item::Rock => Self::Paper,
                Item::Paper => Self::Scissors,
                Item::Scissors => Self::Rock,
            },
            Outcome::Lose => match self {
                Item::Rock => Self::Scissors,
                Item::Paper => Self::Rock,
                Item::Scissors => Self::Paper,
            }
            Outcome::Draw => *self,
        }
    }

    fn does_win_against(&self, other: &Self) -> Outcome {
        match self {
            Item::Rock => match other {
                Item::Rock => Outcome::Draw,
                Item::Paper => Outcome::Lose,
                Item::Scissors => Outcome::Win,
            }
            Item::Paper => match other {
                Item::Rock => Outcome::Win,
                Item::Paper => Outcome::Draw,
                Item::Scissors => Outcome::Lose,
            },
            Item::Scissors => match other {
                Item::Rock => Outcome::Lose,
                Item::Paper => Outcome::Win,
                Item::Scissors => Outcome::Draw,
            },
        }
    }

    fn score(&self) -> u16 {
        match self {
            Item::Rock => 1,
            Item::Paper => 2,
            Item::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_outcome_code(code: impl Into<String>) -> Self {
        match code.into().as_str() {
            "Z" => Self::Win,
            "Y" => Self::Draw,
            _ => Self::Lose,
        }
    }

    fn score(&self) -> u16 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

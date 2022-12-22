use std::{collections::HashSet, error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    let all_priorities: u16 = input
        .split("\n")
        .filter_map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            let mut left_set = HashSet::<char>::new();
            left.chars().for_each(|c| {
                left_set.insert(c);
            });
            for c in right.chars() {
                if left_set.contains(&c) {
                    return Some(c.priority());
                }
            }
            None
        })
        .sum();

    println!("Sum of all prios: {}", all_priorities);

    let sum_of_badge_prios: u16 = input
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .filter_map(|group| {
            let mut sets = group.iter().map(|rucksack| {
                let mut set = HashSet::new();
                rucksack.chars().for_each(|c| {set.insert(c);});
                set
            });

            if let (Some(first), Some(second), Some(third)) = (sets.next(), sets.next(), sets.next()) {
                for c in first {
                    if second.contains(&c) && third.contains(&c) {
                        return Some(c.priority());
                    }
                }
            }
            None
        })
        .sum();

    println!("Sum of badge priorities: {}", sum_of_badge_prios);

    Ok(())
}

impl HasPrio for char {
    fn priority(&self) -> u16 {
        let offset = if self.is_uppercase() { -38 } else { -96 };
        let mut b = [0u8; 1];
        self.encode_utf8(&mut b);
        let val: i16 = b[0].into();
        (val + offset).try_into().unwrap()
    }
}

trait HasPrio {
    fn priority(&self) -> u16;
}

#[cfg(test)]
mod test {
    use crate::HasPrio;

    #[test]
    fn prios() {
        assert_eq!('a'.priority(), 1);
        assert_eq!('z'.priority(), 26);
        assert_eq!('A'.priority(), 27);
        assert_eq!('Z'.priority(), 52);
    }
}

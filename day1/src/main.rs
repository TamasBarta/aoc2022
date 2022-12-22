use std::{fs::File, io::Read, u32};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    let mut elf_calories = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .filter_map(|food| -> Option<u32> {
                    food.parse::<u32>().ok()
                })
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    elf_calories.sort();
    elf_calories.reverse();

    let (first_3, _) = elf_calories.split_at(3);

    let max_elf_calories = first_3.iter().sum::<u32>();

    println!("Max calorires: {}", max_elf_calories);

    Ok(())
}

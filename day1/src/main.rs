fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../input.txt");

    let mut elf_calories: Vec<_> = input
        .split("\n\n")
        .map(|elf| elf.lines().flat_map(str::parse::<usize>).sum())
        .collect();

    println!("Most calories: {}", elf_calories.iter().max().unwrap_or_else(|| &0));

    elf_calories.sort();

    let max_elf_cals: usize = elf_calories.iter().rev().take(3).sum();

    println!("Sum of calories of top 3: {}", max_elf_cals);

    Ok(())
}

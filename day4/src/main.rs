use std::{fs::File, io::Read, ops::RangeInclusive};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;

    let pairs = input
        .split("\n")
        .filter_map(|pair| {
            let mut pair = pair.split(",").filter_map(|elf| {
                let mut split = elf.split("-");
                Some(split.next()?.parse::<u16>().ok()?..=split.next()?.parse::<u16>().ok()?)
            });

            let (first, second) = (pair.next()?, pair.next()?);

            Some((first, second))
        });
    let number_of_contained_pairs = pairs.clone()
        .filter_map(|(first, second)| {
            let contained =
                range_contains_range(&first, &second) || range_contains_range(&second, &first);

            if contained {
                Some(())
            } else {
                None
            }
        })
        .count();
    let number_of_overlapping_pairs = pairs
        .filter_map(|(first, second)| {
            let overlaps =
                range_overlaps_range(&first, &second);

            if overlaps {
                Some(())
            } else {
                None
            }
        })
        .count();

    println!("The number of complete overlaps is: {}", number_of_contained_pairs);
    println!("The number of partial overlaps is: {}", number_of_overlapping_pairs);

    Ok(())
}

fn range_contains_range(range: &RangeInclusive<u16>, other: &RangeInclusive<u16>) -> bool {
    range.start() <= other.start() && range.end() >= other.end()
}

fn range_overlaps_range(range: &RangeInclusive<u16>, other: &RangeInclusive<u16>) -> bool {
    range.end().max(other.end()) - range.start().min(other.start())
        < (range.end() - range.start()) + (other.end() - other.start()) + 1 
}

#[cfg(test)]
mod tests {
    use crate::range_contains_range;
    use crate::range_overlaps_range;

    #[test]
    fn range_contained_tests() {
        assert!(range_contains_range(&(2u16..=5u16), &(3u16..=4u16)));
        assert!(range_contains_range(&(2u16..=5u16), &(2u16..=4u16)));
        assert!(range_contains_range(&(2u16..=5u16), &(3u16..=5u16)));
        assert!(range_contains_range(&(4u16..=4u16), &(4u16..=4u16)));

        assert!(!range_contains_range(&(4u16..=5u16), &(3u16..=5u16)));
        assert!(!range_contains_range(&(4u16..=5u16), &(3u16..=4u16)));
    }

    #[test]
    fn range_overlaps_tests() {
        assert!(range_overlaps_range(&(2u16..=5u16), &(3u16..=4u16)));
        assert!(range_overlaps_range(&(2u16..=5u16), &(2u16..=4u16)));
        assert!(range_overlaps_range(&(2u16..=5u16), &(3u16..=5u16)));
        assert!(range_overlaps_range(&(2u16..=5u16), &(3u16..=6u16)));
        assert!(range_overlaps_range(&(2u16..=5u16), &(1u16..=4u16)));

        assert!(range_overlaps_range(&(4u16..=5u16), &(3u16..=5u16)));
        assert!(range_overlaps_range(&(4u16..=5u16), &(3u16..=4u16)));
        assert!(range_overlaps_range(&(4u16..=4u16), &(4u16..=4u16)));
        assert!(!range_overlaps_range(&(4u16..=4u16), &(5u16..=5u16)));
    }

}

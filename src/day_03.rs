use std::{fs, collections::HashSet};

use self::rucksack::{Rucksack, char_to_value};

mod rucksack;
pub fn day_03() {
    let rucksacks = read_to_rucksacks("./input/day_03.txt");
    println!("shared values: {}", calc_shared_sum(&rucksacks));
    println!("shared group values: {}", calc_shared_group_val(&rucksacks));
}

fn read_to_rucksacks(path: &str) -> Vec<Rucksack> {
    let input = fs::read_to_string(path).unwrap();
    input
        .split('\n')
        .map(|rucksack| rucksack.chars().collect())
        .collect()
}

fn calc_shared_sum(rucksacks: &[Rucksack]) -> u32 {
    rucksacks.iter().map(|rucksack| rucksack.shared_value()).sum()
}

fn calc_shared_group_val(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .chunks(3)
        .map(|group| 
            char_to_value(group
            .iter()
            .fold(HashSet::new(), |shared, rucksack| rucksack.find_matching_chars(shared))
            .iter().next().unwrap())
        ).
        sum()
}

#[cfg(test)]
mod tests {
    use crate::day_03::{calc_shared_sum, calc_shared_group_val};

    use super::read_to_rucksacks;

    #[test]
    fn test_part_1() {
        let rucksacks = read_to_rucksacks("./input/day_03_test.txt");
        assert_eq!(calc_shared_sum(&rucksacks), 157);
    }

    #[test]
    fn test_part_2() {
        let rucksacks = read_to_rucksacks("./input/day_03_test.txt");
        assert_eq!(calc_shared_group_val(&rucksacks), 70);
    }
}
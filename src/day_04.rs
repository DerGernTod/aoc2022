mod clearing_zone;
use std::fs;

use self::clearing_zone::ClearingZone;
fn read_into_zones(path: &str) -> Vec<ClearingZone> {
    let input = fs::read_to_string(path).unwrap();
    let clearing_zone_numbers: Vec<u32> = input
        .split('\n')
        .flat_map(|line| line.split(','))
        .flat_map(|zone| zone.split('-'))
        .map(|begin_or_end| begin_or_end.parse::<u32>().unwrap())
        .collect();
    clearing_zone_numbers
        .chunks(2)
        .map(|chunk| ClearingZone::from_iter(chunk.to_owned()))
        .collect::<Vec<ClearingZone>>()
}

fn count_overlaps(zones: &[ClearingZone]) -> usize {
    zones
        .chunks(2)
        .filter(|chunk| chunk[0].overlaps(&chunk[1]))
        .count()
}

fn count_full_includes(zones: &[ClearingZone]) -> usize {
    zones
        .chunks(2)
        .filter(|chunk| chunk[0].includes_two_way(&chunk[1]))
        .count()
}

pub fn day_04() {
    let zones = read_into_zones("./input/day_04.txt");
    println!("including groups: {}", count_full_includes(&zones));
    println!("overlapping groups: {}", count_overlaps(&zones));
}
#[cfg(test)]
mod tests {
    use super::{read_into_zones, count_full_includes, count_overlaps};
    #[test]
    fn test_part_1() {
        let zones = read_into_zones("./input/day_04.test.txt");
        assert_eq!(count_full_includes(&zones), 2);
    }
    #[test]
    fn test_part_2() {
        let zones = read_into_zones("./input/day_04.test.txt");
        assert_eq!(count_overlaps(&zones), 4);
    }
}
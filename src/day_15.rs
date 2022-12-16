use std::{fs, collections::BTreeSet, ops::Range, time::Instant};

use self::{sensor::Sensor, coords::Coords};

mod sensor;
mod coords;

#[allow(dead_code)]
pub fn day_15() {
    let now = Instant::now();
    let sensors = parse_into_sensors("./input/day_15.txt");
    println!("num covered columns in line {}: {} in {:.2?}", 2000000, calc_num_covered_columns(&sensors, 2000000, i32::MIN, i32::MAX).0, now.elapsed());
    let now = Instant::now();
    let uncovered_coords = calc_uncovered_coords(&sensors, 0, 4000000);
    println!("tuning frequency: {} in {:.2?}", calc_tuning_frequency(&uncovered_coords), now.elapsed());
}

fn parse_into_sensors(path: &str) -> Vec<Sensor> {
    let input = fs::read_to_string(path).unwrap();
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn calc_num_covered_columns(sensors: &[Sensor], row: i32, min: i32, max: i32) -> (u32, Vec<Range<i32>>) {
    let covered_sections: BTreeSet<(i32, i32)> = 
        sensors
            .iter()
            .map(|sensor| sensor.calc_covered_row_start_end(row))
            .filter(|(start, end)| start != end)
            .collect();
    let mut cur_start: Option<i32> = None;
    let mut cur_end = None;
    let mut uncovered_locations = vec![];
    let mut num_covered = 0;
    for (section_start, section_end) in covered_sections {
        if let Some(end) = cur_end {
            if section_start <= end && section_end >= end {
                cur_end = Some(section_end.min(max));
            } else if section_start > end {
                num_covered += cur_start.unwrap().abs_diff(end);
                uncovered_locations.push((end + 1)..section_start);
                cur_start = Some(section_start.max(min));
                cur_end = Some(section_end.min(max));
            }
        }
        if cur_start.is_none() {
            cur_start = if section_start < min && section_end >= min {
                Some(min)
            } else if section_start < min {
                None
            } else {
                uncovered_locations.push(min..section_start);
                Some(section_start)
            }
        }
        if cur_end.is_none() {
            cur_end = if section_start < max && section_end >= max {
                Some(max)
            } else if section_start < max {
                Some(section_end)
            } else {
                None
            }
        }
    }
    (num_covered + cur_end.unwrap().abs_diff(cur_start.unwrap()), uncovered_locations)
}

fn calc_uncovered_coords(sensors: &[Sensor], min: i32, max: i32) -> Coords {
    for y in min..=max {
        let cover_result = calc_num_covered_columns(sensors, y, min, max);
        if cover_result.1.len() == 1 && cover_result.1.get(0).unwrap().len() == 1 {
            return Coords(cover_result.1.get(0).unwrap().start, y);
        }
    }
    panic!("Didn't find an uncovered coord!");
}

fn calc_tuning_frequency(coords: &Coords) -> i64 {
    coords.0 as i64 * 4000000 + coords.1 as i64
}

#[cfg(test)]
mod tests {
    use crate::day_15::{calc_uncovered_coords, coords::Coords, calc_tuning_frequency};

    use super::{parse_into_sensors, calc_num_covered_columns};

    #[test]
    fn test_part_1() {
        let sensors = parse_into_sensors("./input/day_15.test.txt");
        assert_eq!(calc_num_covered_columns(&sensors, 10, i32::MIN, i32::MAX).0, 26);
    }
    #[test]
    fn test_part_2() {
        let sensors = parse_into_sensors("./input/day_15.test.txt");
        
        let uncovered_result = calc_num_covered_columns(&sensors, 11, 0, 20);
        println!("{:?}", uncovered_result);
        assert_eq!(uncovered_result.0, 18);
        assert_eq!(uncovered_result.1.get(0).unwrap().len(), 1);
        let uncovered_coords = calc_uncovered_coords(&sensors, 0, 20);
        assert_eq!(calc_tuning_frequency(&uncovered_coords), 56000011);
    }
}
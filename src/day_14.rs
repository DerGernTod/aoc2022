mod coords;

use std::{collections::HashSet, fs};

use self::coords::Coords;

pub fn day_14() {
    let cave = parse_into_coords("./input/day_14.txt");
    println!("resting sand: {}", count_resting_sand(cave));

    
    let cave = parse_into_coords("./input/day_14.txt");
    println!("resting sand with floor: {}", count_resting_sand_with_floor(cave));
}

fn collect_between(coords: &[Coords]) -> Vec<Coords> {
    let start = &coords[0];
    let end = &coords[1];
    let mut between = vec![];
    if start.0 == end.0 {
        for y in start.1.min(end.1)..start.1.max(end.1) {
            between.push(Coords(start.0, y));
        }
    } else {
        for x in start.0.min(end.0)..start.0.max(end.0) {
            between.push(Coords(x, start.1));
        }
    }
    between
}

fn parse_into_coords(path: &str) -> HashSet<Coords> {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .flat_map(|line| -> HashSet<Coords> {
            let mut vec_coords: Vec<Coords> = line
                .split(" -> ")
                .map(|coord| {
                    let mut nums = coord.split(',').map(|num| num.parse::<i32>().unwrap());
                    Coords(nums.next().unwrap(), nums.next().unwrap())
                })
                .collect();
            vec_coords.extend(vec_coords.windows(2).flat_map(collect_between).collect::<Vec<Coords>>());
            HashSet::from_iter(vec_coords.into_iter())
        })
        .collect()
}

fn calc_abyss_location(cave: &HashSet<Coords>) -> &i32 {
    cave
        .iter()
        .map(|Coords(_, y)| y)
        .max()
        .unwrap()

}

fn drop_sand(cave: &mut HashSet<Coords>, abyss_location: i32) -> bool {
    let mut cur_pos = Coords(500, 0);
    while cur_pos.1 < abyss_location {
        let lower_pos = Coords(cur_pos.0, cur_pos.1 + 1);
        let diag_left_pos = Coords(cur_pos.0 - 1, cur_pos.1 + 1);
        let diag_right_pos = Coords(cur_pos.0 + 1, cur_pos.1 + 1);
        match (cave.contains(&lower_pos), cave.contains(&diag_left_pos), cave.contains(&diag_right_pos)) {
            (false, _, _) => cur_pos = lower_pos,
            (true, false, _) => cur_pos = diag_left_pos,
            (true, true, false) => cur_pos = diag_right_pos,
            (true, true, true) => {
                cave.insert(cur_pos);
                return true;
            },
        }
    }
    false
}

fn drop_sand_with_floor(cave: &mut HashSet<Coords>, floor_location: i32) -> bool {
    let mut cur_pos = Coords(500, 0);
    if cave.contains(&cur_pos) {
        return false;
    }
    loop {
        let lower_pos = Coords(cur_pos.0, cur_pos.1 + 1);
        let diag_left_pos = Coords(cur_pos.0 - 1, cur_pos.1 + 1);
        let diag_right_pos = Coords(cur_pos.0 + 1, cur_pos.1 + 1);
        if cur_pos.1 + 1 == floor_location {
            cave.insert(cur_pos);
            return true;
        }
        match (cave.contains(&lower_pos), cave.contains(&diag_left_pos), cave.contains(&diag_right_pos)) {
            (false, _, _) => cur_pos = lower_pos,
            (true, false, _) => cur_pos = diag_left_pos,
            (true, true, false) => cur_pos = diag_right_pos,
            (true, true, true) => {
                cave.insert(cur_pos);
                return true;
            },
        }
    }
}

fn count_resting_sand(mut cave: HashSet<Coords>) -> usize {
    let abyss_location = *calc_abyss_location(&cave);
    let mut dropped_sand_count = 0;
    while drop_sand(&mut cave, abyss_location) {
        dropped_sand_count += 1;
    }
    dropped_sand_count
}

fn count_resting_sand_with_floor(mut cave: HashSet<Coords>) -> usize {
    let abyss_location = *calc_abyss_location(&cave);
    let mut dropped_sand_count = 0;
    while drop_sand_with_floor(&mut cave, abyss_location + 2) {
        dropped_sand_count += 1;
    }
    dropped_sand_count
}

#[cfg(test)]
mod tests {
    use crate::day_14::{coords::Coords, calc_abyss_location, count_resting_sand_with_floor};

    use super::{parse_into_coords, count_resting_sand};

    #[test]
    fn test_parse_into_coords() {
        let parsed = parse_into_coords("./input/day_14.test.txt");
        assert!(parsed.contains(&Coords(498, 4)));
        assert!(parsed.contains(&Coords(498, 5)));
        assert!(parsed.contains(&Coords(498, 6)));
        assert!(parsed.contains(&Coords(503, 4)));
        assert!(parsed.contains(&Coords(502, 4)));

        assert_eq!(calc_abyss_location(&parsed), &9);
    }

    #[test]
    fn test_part_1() {
        let cave = parse_into_coords("./input/day_14.test.txt");
        assert_eq!(count_resting_sand(cave), 24);
    }

    #[test]
    fn test_part_2() {
        let cave = parse_into_coords("./input/day_14.test.txt");
        assert_eq!(count_resting_sand_with_floor(cave), 93);
    }
}
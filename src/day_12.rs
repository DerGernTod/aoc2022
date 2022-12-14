use std::{collections::{HashMap, HashSet}, fs, num};

#[derive(Copy, Clone)]
struct Location {
    visited: bool,
    height: i32,
    coords: Coords,
    cheapest_path: usize,
}
impl Location {
    fn new(height: i32, coords: Coords) -> Location {
        Location { visited: false, height, coords, cheapest_path: usize::MAX }
    }
}
#[allow(dead_code)]
pub fn day_12() {
    let mut map = parse_into_map("./input/day_12.txt");
    let (start, end) = find_start_and_end(&mut map);
    let required_steps = move_to_goal(&mut map, start, Some(end), false);
    println!("required steps: {}", required_steps.unwrap());

    
    let mut map = parse_into_map("./input/day_12.txt");
    let start_end = find_start_and_end(&mut map);
    move_to_goal(&mut map, start_end.1, None, true);
    println!("required steps to best a: {}", find_cheapest_a_node(&map).unwrap());
}

type Coords = (usize, usize);
type HeightMap = HashMap<Coords, Location>;

fn update_neighbor_distances(map: &mut HeightMap, cur_location: &Coords, reverse: bool) -> usize {
    let (x, y) = *cur_location;

    let self_height = map.get(cur_location).unwrap().height;
    let cur_cost = map.get(cur_location).unwrap().cheapest_path;
    if let Some(location) = map.get_mut(cur_location) {
        location.visited = true
    }
    let mut neighbors = vec![];
    
    if x != 0 { neighbors.push((x - 1, y)) };
    if y != 0 { neighbors.push((x, y - 1)) };
    neighbors.push((x, y + 1));
    neighbors.push((x + 1, y));

    for neighbor in neighbors {
        let neighbor = map.get_mut(&neighbor);
        if let Some(neighbor) = neighbor {
            let can_move = self_height >= neighbor.height - 1;
            let can_move_reverse = self_height <= neighbor.height + 1;
            if (can_move && !reverse) || (reverse && can_move_reverse)  {
                neighbor.cheapest_path = neighbor.cheapest_path.min(cur_cost + 1);
            }
        }
    }
    cur_cost
}

fn move_to_goal(map: &mut HeightMap, start: Coords, end: Option<Coords>, reverse: bool) -> Option<usize> {
    let mut num_steps;
    let mut cur_pos = start;
    if let Some(start_pos) = map.get_mut(&start) {
        start_pos.cheapest_path = 0;
    }
    let mut remaining_steps: HashSet<Coords> = map.clone().into_keys().collect();
    while !remaining_steps.is_empty() {
        num_steps = update_neighbor_distances(map, &cur_pos, reverse);
        if Some(cur_pos) == end {
            return Some(num_steps);
        }
        remaining_steps.remove(&cur_pos);
        let mut lowest = (None, usize::MAX);
        remaining_steps.iter().for_each(|coords| {
            if lowest.0.is_some() {
                let location = map.get(coords).unwrap();
                if location.cheapest_path < lowest.1 {
                    lowest = (Some(coords), location.cheapest_path);
                }
            } else {
                lowest = (Some(coords), map.get(coords).unwrap().cheapest_path);
            }
        });

        if let Some(lowest_coord) = lowest.0 {
            cur_pos = *lowest_coord;
            if lowest.1 == usize::MAX {
                return None;
                // panic!("Path blocked everywhere!");
            }
        } else {
            return None;
        }
    }
    None
}

fn find_cheapest_a_node(map: &HeightMap) -> Option<usize> {

    let mut a_nodes: Vec<&Location> = map
        .values()
        .filter(|location| location.height == 'a' as i32)
        .collect();
    a_nodes.sort_by_key(|location| location.cheapest_path);

    map
        .values()
        .filter(|location| location.height == 'a' as i32)
        .map(|location| location.cheapest_path)
        .min()
}

fn parse_into_map(path: &str) -> HashMap<Coords, Location> {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| line
            .char_indices()
            .map(move |(col, ch)| ((col, row), Location::new(ch as i32, (col, row)))))
        .collect()
}

fn find_start_and_end(map: &mut HeightMap) -> (Coords, Coords) {
    let mut start_end = (None, None);
    
    for location in map.values_mut() {
        if location.height == 'S' as i32 {
            location.height = 'a' as i32;
            start_end.0 = Some(location.coords);
        } else if location.height == 'E' as i32 {
            location.height = 'z' as i32;
            start_end.1 = Some(location.coords);
        }
        if let Some(start) = start_end.0 {
            if let Some(end) = start_end.1 {
                return (start, end);
            }
        }
    }
    panic!("Didn't find start and end!")
}

#[cfg(test)]
mod tests {
    use crate::day_12::move_to_goal;

    use super::{parse_into_map, find_start_and_end, find_cheapest_a_node};

    #[test]
    fn test_part_1() {
        let mut map = parse_into_map("./input/day_12.test.txt");
        let start_end = find_start_and_end(&mut map);
        assert_eq!(start_end, ((0, 0), (5, 2)));
        let required_steps = move_to_goal(&mut map, start_end.0, Some(start_end.1), false);
        assert_eq!(required_steps, Some(31));
    }

    #[test]
    fn test_part_2() {
        let mut map = parse_into_map("./input/day_12.test.txt");
        let start_end = find_start_and_end(&mut map);
        move_to_goal(&mut map, start_end.1, None, true);
        assert_eq!(find_cheapest_a_node(&map), Some(29));
    }
}
use std::{fs, collections::HashMap};

#[allow(dead_code)]
pub fn day_08() {
    let map = read_into_map("./input/day_08.txt");
    println!("Visible trees: {}", num_visible_trees(&map));
    println!("Highest scenic: {}", calc_highest_scenic_score(&map));
}

type Coords = (i32, i32);
type ForestMap = HashMap<Coords, u32>;

fn read_into_map(path: &str) -> ForestMap {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| line
            .chars()
            .enumerate()
            .map(move |(col, num)| ((row as i32, col as i32), num.to_digit(10).unwrap())))
        .collect()
}

fn calc_highest_scenic_score(map: &ForestMap) -> u32 {
    map.iter().map(|(coords, _)| calc_tree_scenic_score(map, coords)).max().unwrap()
}

fn calc_tree_scenic_score(map: &ForestMap, coords: &Coords) -> u32 {
    let tree_height = map.get(coords).unwrap();
    let (row, col) = *coords;
    let total_width = f32::sqrt(map.len() as f32) as i32;

    let mut visible_trees_left = 0;
    for check_col in (0..col).rev() {
        visible_trees_left += 1;
        if map.get(&(row, check_col)).unwrap() >= tree_height {
            break;
        } 
    }

    let mut visible_trees_right = 0;
    for check_col in (col + 1)..total_width {
        visible_trees_right += 1;
        if map.get(&(row, check_col)).unwrap() >= tree_height {
            break;
        }
    }

    let mut visible_trees_top = 0;
    for check_row in (0..row).rev() {
        visible_trees_top += 1;
        if map.get(&(check_row, col)).unwrap() >= tree_height {
            break;
        }
    }

    let mut visible_trees_bottom = 0;
    for check_row in (row + 1)..total_width {
        visible_trees_bottom += 1;
        if map.get(&(check_row, col)).unwrap() >= tree_height {
            break;
        }
    }
    visible_trees_bottom
    * visible_trees_top
    * visible_trees_left
    * visible_trees_right
}

fn is_tree_hidden(map: &ForestMap, coords: &Coords) -> bool {
    let tree_height = map.get(coords).unwrap();
    let (row, col) = *coords;
    let total_width = f32::sqrt(map.len() as f32) as i32;

    let mut left_obstructed = false;
    for check_col in 0..col {
        if map.get(&(row, check_col)).unwrap() >= tree_height {
            left_obstructed = true;
            break;
        } 
    }

    let mut right_obstructed = false;
    for check_col in (col + 1)..total_width {
        if map.get(&(row, check_col)).unwrap() >= tree_height {
            right_obstructed = true;
            break;
        }
    }

    let mut top_obstructed = false;
    for check_row in 0..row {
        if map.get(&(check_row, col)).unwrap() >= tree_height {
            top_obstructed = true;
            break;
        }
    }

    let mut bot_obstructed = false;
    for check_row in (row + 1)..total_width {
        if map.get(&(check_row, col)).unwrap() >= tree_height {
            bot_obstructed = true;
            break;
        }
    }
    left_obstructed && right_obstructed && bot_obstructed && top_obstructed
}

fn num_visible_trees(map: &ForestMap) -> usize {
    map.iter().filter( |(coords, _)| !is_tree_hidden(map, coords)).count()
}

#[cfg(test)]
mod tests {
    use crate::day_08::calc_highest_scenic_score;

    use super::{read_into_map, num_visible_trees, is_tree_hidden, calc_tree_scenic_score};

    #[test]
    fn test_calc_tree_scenic_score() {
        let map = read_into_map("./input/day_08.test.txt");
        assert_eq!(calc_tree_scenic_score(&map, &(1, 2)), 4);
        assert_eq!(calc_tree_scenic_score(&map, &(3, 2)), 8);
    }
    #[test]
    fn test_is_tree_hidden() {
        let map = read_into_map("./input/day_08.test.txt");
        // borders are always visible
        assert!(!is_tree_hidden(&map, &(0, 0)));
        assert!(!is_tree_hidden(&map, &(0, 1)));
        assert!(!is_tree_hidden(&map, &(0, 2)));
        assert!(!is_tree_hidden(&map, &(0, 3)));
        assert!(!is_tree_hidden(&map, &(0, 4)));

        assert!(!is_tree_hidden(&map, &(4, 0)));
        assert!(!is_tree_hidden(&map, &(4, 1)));
        assert!(!is_tree_hidden(&map, &(4, 2)));
        assert!(!is_tree_hidden(&map, &(4, 3)));
        assert!(!is_tree_hidden(&map, &(4, 4)));

        assert!(!is_tree_hidden(&map, &(1, 0)));
        assert!(!is_tree_hidden(&map, &(2, 0)));
        assert!(!is_tree_hidden(&map, &(3, 0)));

        assert!(!is_tree_hidden(&map, &(0, 1)));
        assert!(!is_tree_hidden(&map, &(0, 2)));
        assert!(!is_tree_hidden(&map, &(0, 3)));

        // check inner trees
        assert!(!is_tree_hidden(&map, &(1, 1)));
        assert!(!is_tree_hidden(&map, &(1, 2)));
        assert!(is_tree_hidden(&map, &(1, 3)));

        assert!(!is_tree_hidden(&map, &(2, 1)));
        assert!(is_tree_hidden(&map, &(2, 2)));
        assert!(!is_tree_hidden(&map, &(2, 3)));

        assert!(is_tree_hidden(&map, &(3, 1)));
        assert!(!is_tree_hidden(&map, &(3, 2)));
        assert!(is_tree_hidden(&map, &(3, 3)));
        

    }

    #[test]
    fn test_part_1() {
        let map = read_into_map("./input/day_08.test.txt");
        assert_eq!(num_visible_trees(&map), 21);
    }

    #[test]
    fn test_part_2() {
        let map = read_into_map("./input/day_08.test.txt");
        assert_eq!(calc_highest_scenic_score(&map), 8);
    }
}
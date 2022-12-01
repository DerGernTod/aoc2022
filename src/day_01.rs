use std::fs;


fn read_to_bags(path: &str) -> Vec<Vec<u32>> {
    let input = fs::read_to_string(path).unwrap();
    input.split("\n\n")
        .map(|bag| bag.split('\n').map(|entry| entry.parse::<u32>().unwrap()).collect())
        .collect()
}

fn get_largest_bag_count(bags: &Vec<Vec<u32>>) -> u32 {
    let mut counts: Vec<u32> = bags.iter().map(|bag| bag.iter().sum()).collect();
    counts.sort();
    *counts.last().unwrap()
}

fn get_top_three_count(bags: &Vec<Vec<u32>>) -> u32 {
    let mut counts: Vec<u32> = bags.iter().map(|bag| bag.iter().sum()).collect();
    counts.sort();
    counts.pop().unwrap() + counts.pop().unwrap() + counts.pop().unwrap()
}

pub fn day_01() {
    
    let bags = read_to_bags("./input/day_01.txt");
    println!("thickest bag: {}", get_largest_bag_count(&bags));
    println!("top 3 bags: {}", get_top_three_count(&bags));
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::day_01::*;

    #[test]
    fn test_part_1() {
        let bags = read_to_bags("./input/day_01.test.txt");
        assert_eq!(bags, vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000, 8000, 9000], vec![10000]]);
        assert_eq!(get_largest_bag_count(&bags), 24000);
    }

    #[test]
    fn test_part_2() {
        let bags = read_to_bags("./input/day_01.test.txt");
        assert_eq!(get_top_three_count(&bags), 45000);
    }
}
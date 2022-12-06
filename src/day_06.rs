use std::{fs, collections::HashSet};

pub fn day_06() {
    println!("first proto at {}", parse_into_protocol_start("./input/day_06.txt", 4));
    println!("first message at {}", parse_into_protocol_start("./input/day_06.txt", 14));
}

fn parse_into_protocol_start(path: &str, num_consecutive_chars: usize) -> usize {
    let str = fs::read_to_string(path).unwrap();
    for (i, _) in str.chars().enumerate() {
        let signal: HashSet<char> = HashSet::from_iter(str.chars().skip(i).take(num_consecutive_chars));
        if signal.len() == num_consecutive_chars {
            return i + num_consecutive_chars;
        }
    }
    panic!("Didn't find 4 different chars!");
}

#[cfg(test)]
mod tests {
    use super::parse_into_protocol_start;

    #[test]
    fn test_part_1() {
        assert_eq!(parse_into_protocol_start("./input/day_06.test.txt", 4), 5);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(parse_into_protocol_start("./input/day_06.test.txt", 14), 23);
    }
}
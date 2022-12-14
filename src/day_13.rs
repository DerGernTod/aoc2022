use std::{fs, str::Chars, cmp::Ordering};

use self::signal_content::SignalContent;

mod signal_content;

#[allow(dead_code)]
pub fn day_13() {
    let pairs = parse_into_pairs("./input/day_13.txt");
    println!("ordered item indices: {}", calc_ordered_index_sum(&pairs));

    let mut signals = parse_into_pair_contents("./input/day_13.txt");
    signals.sort();
    let divider_index_product = calc_divider_index_product(&signals);
    println!("divider index product is: {divider_index_product}");
}

fn digits_chars_to_usize(digits: &[char]) -> usize {
    digits.iter().collect::<String>().parse::<usize>().unwrap()
}

fn parse_block(chars_iter: &mut Chars) -> SignalContent {
    let mut cur_digits = vec![];
    let mut cur_pair_content = vec![];
    while let Some(ch) = chars_iter.next() {
        match ch {
            ']' => {
                if !cur_digits.is_empty() {
                    cur_pair_content.push(SignalContent::Number(digits_chars_to_usize(&cur_digits)))
                }
                return SignalContent::Block(cur_pair_content);
            },
            '[' => cur_pair_content.push(parse_block(chars_iter)),

            '0'..='9' => {
                cur_digits.push(ch);
            },
            ',' => {
                if !cur_digits.is_empty() {
                    cur_pair_content.push(SignalContent::Number(digits_chars_to_usize(&cur_digits)))
                }
                cur_digits.clear();
            }
            _ => panic!("Unexpected character: {ch}")
        }
    }
    SignalContent::Block(cur_pair_content)
}


fn parse_into_pairs(path: &str) -> Vec<(usize, (SignalContent, SignalContent))> {
    let input = fs::read_to_string(path).unwrap();
    input.split("\n\n")
        .into_iter()
        .map(|block| {
            let mut lines = block.lines();
            let lhs = parse_block(&mut lines.next().unwrap().chars());
            let rhs = parse_block(&mut lines.next().unwrap().chars());
            (lhs, rhs)
        })
        .enumerate()
        .collect()
}

fn calc_ordered_index_sum(signals: &[(usize, (SignalContent, SignalContent))]) -> usize {
    signals
        .iter()
        .filter(|(_, (a, b))| a < b)
        .map(|(index, _)| index + 1)
        .sum()
}

fn calc_divider_index_product(signals: &[SignalContent]) -> usize {
    let (first, second) = &SignalContent::generate_dividers();
    signals
        .iter()
        .enumerate()
        .filter(|(_, signal)| signal == &first || signal == &second)
        .map(|(index, _)| index + 1)
        .product()
}

fn parse_into_pair_contents(path: &str) -> Vec<SignalContent> {
    let input = fs::read_to_string(path).unwrap();
    let mut all: Vec<SignalContent> = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_block(&mut line.chars()))
        .collect();
    let (first, second) = SignalContent::generate_dividers();
    all.push(first);
    all.push(second);
    all
}

#[cfg(test)]
mod tests {

    use crate::day_13::{parse_into_pair_contents, calc_ordered_index_sum};

    use super::{parse_into_pairs, signal_content::SignalContent, calc_divider_index_product};

    #[test]
    fn test_part_2() {
        let mut pairs = parse_into_pair_contents("./input/day_13.test.txt");
        pairs.sort();
        assert_eq!(calc_divider_index_product(&pairs), 140);
    }
    #[test]
    fn test_part_1() {
        let pairs = parse_into_pairs("./input/day_13.test.txt");
        assert_eq!(calc_ordered_index_sum(&pairs), 13);
    }
}

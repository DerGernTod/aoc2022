use std::{fs, borrow::BorrowMut};

use self::monkey::{Monkey, Operator};

mod monkey;

#[allow(dead_code)]
pub fn day_11() {
    let monkeys = read_into_monkeys("./input/day_11.txt");
    let monkeys_after_rounds = exec_num_rounds(monkeys, 20, None);
    println!("most active monkeys items: {}", multiply_highest_handled(&monkeys_after_rounds));
    
    let monkeys = read_into_monkeys("./input/day_11.txt");
    let common_denominator = find_common_denominator(&monkeys);
    let monkeys_after_rounds = exec_num_rounds(monkeys, 10000, Some(common_denominator));
    println!("most active monkeys items after 10000 rounds: {}", multiply_highest_handled(&monkeys_after_rounds));
}

fn find_common_denominator(monkeys: &[Monkey]) -> usize {
    monkeys.iter().map(|monkey| monkey.get_divisor()).product()
}

fn read_into_monkeys(path: &str) -> Vec<Monkey> {
    let input = fs::read_to_string(path).unwrap();
    input
        .split("\n\n")
        .enumerate()
        .map(|(id, monkey_section)| {
            let mut lines = monkey_section.lines().skip(1);
            // collect items
            let items: Vec<usize> = lines
                .next()
                .unwrap()
                .split(": ")
                .last()
                .unwrap()
                .split(", ")
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            let mut operation = lines
                .next()
                .unwrap()
                .split("= ")
                .last()
                .unwrap()
                .split(' ');
            let lhs = operation.next().unwrap();
            let operator = operation.next().unwrap();
            let rhs = operation.next().unwrap();
            let op_closure = match (lhs, operator, rhs) {
                ("old", "+", "old") => Operator::Add(None),
                ("old", "*", "old") => Operator::Multiply(None),
                ("old", "+", x) => Operator::Add(Some(x.parse::<usize>().unwrap())),
                ("old", "*", x) => Operator::Multiply(Some(x.parse::<usize>().unwrap())),
                _ => panic!("Unknown command: {lhs} {operator} {rhs}")
            };
            let test = lines.next().unwrap().split(' ').last().unwrap().parse::<usize>().unwrap();
            let true_monkey = lines.next().unwrap().split(' ').last().unwrap().parse::<usize>().unwrap();
            let false_monkey = lines.next().unwrap().split(' ').last().unwrap().parse::<usize>().unwrap();
            Monkey::new(id, items, (test, true_monkey, false_monkey), op_closure)
        })
        .collect()
}

fn exec_num_rounds(mut monkeys: Vec<Monkey>, rounds: usize, worry_divisor: Option<usize>) -> Vec<Monkey> {
    for _ in 0..rounds {
        for monkey_id in 0..monkeys.len() {
            let commands = monkeys.get_mut(monkey_id).unwrap().exec_round(worry_divisor);
            for (monkey_id, item) in commands {
                monkeys.get_mut(monkey_id).unwrap().add_item(item);
            }
        }
    }
    monkeys
}

fn multiply_highest_handled(monkeys: &[Monkey]) -> usize {
    let mut num_handled: Vec<usize> = monkeys
        .iter()
        .map(|monkey| monkey.handled_items)
        .collect();
    num_handled.sort();
    num_handled.iter()
        .rev()
        .take(2)
        .product()
}

#[cfg(test)]
mod tests {

    use crate::day_11::find_common_denominator;

    use super::{read_into_monkeys, exec_num_rounds, multiply_highest_handled, monkey::{Monkey, Operator}};

    #[test]
    fn test_part_1() {
        let monkeys = read_into_monkeys("./input/day_11.test.txt");
        assert_eq!(monkeys[0], Monkey::new(0, vec![79, 98], (23, 2, 3), Operator::Multiply(Some(19))));
        let monkeys_after_rounds = exec_num_rounds(monkeys, 20, None);
        assert_eq!(multiply_highest_handled(&monkeys_after_rounds), 10605);
    }
    #[test]
    fn test_part_2() {
        let monkeys = read_into_monkeys("./input/day_11.test.txt");
        assert_eq!(monkeys[0], Monkey::new(0, vec![79, 98], (23, 2, 3), Operator::Multiply(Some(19))));
        let common_denominator = find_common_denominator(&monkeys);
        let monkeys_after_rounds = exec_num_rounds(monkeys, 10000, Some(common_denominator));
        assert_eq!(multiply_highest_handled(&monkeys_after_rounds), 2713310158);
    }
}
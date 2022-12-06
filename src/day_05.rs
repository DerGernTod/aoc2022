use std::{fs, collections::BTreeMap};
#[allow(dead_code)]
pub fn day_05() {
    let (mut stacks, commands) = parse_into_stacks_and_commands("./input/day_05.txt");
    let mut stacks_9001 = stacks.clone();
    apply_commands_on_stacks(&mut stacks, &commands);
    println!("top crates: {:?}", find_top_crates(&stacks).iter().collect::<String>());
    apply_commands_on_stacks_9001(&mut stacks_9001, &commands);
    println!("top crates 9001: {:?}", find_top_crates(&stacks_9001).iter().collect::<String>());
}

type StacksAndCommands = (BTreeMap<usize, Vec<char>>, Vec<(usize, usize, usize)>);

fn parse_into_stacks_and_commands(path: &str) -> StacksAndCommands  {
    let input = fs::read_to_string(path).unwrap();
    let mut split = input.split("\n\n");
    let stacks: BTreeMap<usize, Vec<char>> = split.next().unwrap()
        .split('\n')
        .rev()
        // skip the line with stacks label
        .skip(1)
        // map each line to its chars per stack - every 4 chars is a stack entry, skip the first one
        .map(|line| line.chars().skip(1).step_by(4).enumerate())
        // create hashmap of stacks
        .fold(BTreeMap::new(), |mut stacks, entries| {
            for (i, entry) in entries {
                if entry != ' ' {
                    stacks.entry(i + 1).or_default().push(entry);
                }
            }
            stacks
        });
    let commands: Vec<(usize, usize, usize)> = split.next().unwrap().split('\n')
        // map each line to 3 numbers - split to words, skip first, then pick every other
        .map(|line| line.split(' ').skip(1).step_by(2).map(|str| str.parse::<usize>().unwrap()))
        // map each triplet of numbers to a command tuple
        .map(|mut cmd_iter|  (
            cmd_iter.next().unwrap(),
            cmd_iter.next().unwrap(), 
            cmd_iter.next().unwrap()))
        .collect();
    (stacks, commands)
}

fn apply_commands_on_stacks(stacks: &mut BTreeMap<usize, Vec<char>>, commands: &Vec<(usize, usize, usize)>) {
    for (amount, from, to) in commands {
        for _ in 0..*amount {
            let stack = stacks.get_mut(from).unwrap();
            let ch = stack.pop().unwrap();
            let stack = stacks.get_mut(to).unwrap();
            stack.push(ch);
        }
    }
}

fn apply_commands_on_stacks_9001(stacks: &mut BTreeMap<usize, Vec<char>>, commands: &Vec<(usize, usize, usize)>) {
    for (amount, from, to) in commands {
        let start = stacks.get(from).unwrap().len() - amount;
        let drain = stacks.get_mut(from).unwrap().drain(start..);
        let drained: Vec<char> = drain.collect();
        stacks.get_mut(to).unwrap().extend(drained);
    }
}

fn find_top_crates(stacks: &BTreeMap<usize, Vec<char>>) -> Vec<char> {
    stacks.values().map(|stack| stack.last().unwrap()).copied().collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::day_05::{apply_commands_on_stacks, find_top_crates, apply_commands_on_stacks_9001};

    use super::parse_into_stacks_and_commands;

    #[test]
    pub fn test_part_1() {
        let (mut stacks, commands) = parse_into_stacks_and_commands("./input/day_05.test.txt");
        let mut expected_map = BTreeMap::new();
        expected_map.insert(1, vec!['Z', 'N']);
        expected_map.insert(2, vec!['M', 'C', 'D']);
        expected_map.insert(3, vec!['P']);
        assert_eq!(stacks, expected_map);
        assert_eq!(commands, vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)]);

        apply_commands_on_stacks(&mut stacks, &commands);
        let mut expected_map = BTreeMap::new();
        expected_map.insert(1, vec!['C']);
        expected_map.insert(2, vec!['M']);
        expected_map.insert(3, vec!['P', 'D', 'N', 'Z']);
        assert_eq!(find_top_crates(&stacks), vec!['C', 'M', 'Z']);
    }
    #[test]
    pub fn test_part_2() {
        let (mut stacks, commands) = parse_into_stacks_and_commands("./input/day_05.test.txt");

        apply_commands_on_stacks_9001(&mut stacks, &commands);
        let mut expected_map = BTreeMap::new();
        expected_map.insert(1, vec!['M']);
        expected_map.insert(2, vec!['C']);
        expected_map.insert(3, vec!['P', 'Z', 'N', 'D']);
        assert_eq!(find_top_crates(&stacks), vec!['M', 'C', 'D']);
    }
}
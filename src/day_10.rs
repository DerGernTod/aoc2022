use std::fs;

#[allow(dead_code)]
pub fn day_10() {
    let instructions = read_into_instructions("./input/day_10.txt");
    let strength = find_signal_strength_for_instructions(&instructions);
    println!("Total signal strength: {}", strength);
    render_instructions(&instructions);
}

fn find_signal_strength_for_instructions(instructions: &[Option<i32>]) -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    let mut signal_strengths = 0;
    for instruction in instructions {
        cycle += 1;
        signal_strengths += calc_cycle_value(cycle, x);
        if let Some(val) = instruction {
            cycle += 1;
            signal_strengths += calc_cycle_value(cycle, x);
            x += val;
        }
    }
    signal_strengths
}

fn render_instructions(instructions: &[Option<i32>]) {
    let mut cycle = 0;
    let mut x = 1;
    
    for instruction in instructions {
        cycle += 1;
        print!("{}", calc_output(cycle, x));
        if let Some(val) = instruction {
            cycle += 1;
            print!("{}", calc_output(cycle, x));
            x += val;
        }
    }
}

fn calc_output(cycle: i32, signal: i32) -> String {
    let mut output = vec![];
    if [signal - 1, signal, signal + 1].contains(&((cycle - 1) % 40)) {
        output.push('#');
    } else {
        output.push('.');
    }
    if cycle % 40 == 0 {
        output.push('\n');
    }
    output.into_iter().collect()
}

fn calc_cycle_value(cycle: i32, signal: i32) -> i32 {
    match cycle {
        20 => signal * 20i32,
        x if (x - 20) % 40 == 0 => signal * x,
        _ => 0
    }
}

fn read_into_instructions(path: &str) -> Vec<Option<i32>> {
    let input = fs::read_to_string(path).unwrap();

    input.lines()
        .map(|line| line
            .split(' ')
            .nth(1)
            .map(|str| str.parse::<i32>().unwrap_or_else(|_| panic!("Error parsing {} into i32!", str))))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{read_into_instructions, find_signal_strength_for_instructions, calc_cycle_value};

    #[test]
    fn test_read_into_instructions() {
        let instr = read_into_instructions("./input/day_10.test.txt");
        assert_eq!(instr[0], Some(15));
        assert_eq!(instr[1], Some(-11));
        assert_eq!(instr[26], None);
        assert_eq!(instr.len(), 146);
    }

    #[test]
    fn test_calc_cycle_value() {
        assert_eq!(calc_cycle_value(19, 223), 0);
        assert_eq!(calc_cycle_value(20, 21), 420);
        assert_eq!(calc_cycle_value(60, 19), 1140);
    }
    #[test]
    fn test_part_1() {
        let instructions = read_into_instructions("./input/day_10.test.txt");
        let strength = find_signal_strength_for_instructions(&instructions);
        assert_eq!(strength, 13140);
    }
}
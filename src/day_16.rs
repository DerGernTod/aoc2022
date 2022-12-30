use std::{collections::{HashMap, BinaryHeap}, fs, str::FromStr, cell::RefCell, rc::Rc};

use self::{valve::{Valve, ValveTarget}, path_node::PathNode};
/*
 C -- D -- E -- F -- G -- H
 |    |
 B -- A -- I -- J
*/
mod valve;
mod path_node;

pub fn day_16() {
    let pressure = calc_most_released_pressure("./input/day_16.txt", 30);
    println!("I'm able to release up to {pressure} pressure!");
}

fn calc_most_released_pressure(path: &str, minutes: usize) -> usize {
    let valves = parse_into_valves(path);
    let aa_valve = valves.get(&String::from("AA")).unwrap();
    let mut all_paths: HashMap<usize, usize> = HashMap::new();
    all_paths.extend(valves
        .values()
        .filter(|valve| valve.get_flow_rate() > 0)
        .enumerate()
        .map(|(index, _)| (index + 1, usize::MAX)));
    all_paths.insert(all_paths.len() + 1, usize::MAX);
    let mut remaining_valves = BinaryHeap::new();
    remaining_valves.push(PathNode {
        pressure_level: 0,
        total_steps: 0,
        visited: vec![aa_valve.clone()],
        move_cost: 0
    });
    let mut max = 0;
    while let Some(path_node) = remaining_valves.pop() {
        // if all_paths.get(&(path_node.visited.len())).unwrap() < &path_node.move_cost {
        //     continue;
        // }
        // println!("Assigning new min move cost to valve {}: {}",
        //     path_node.most_recent().0.borrow().name,
        //     path_node.move_cost
        //     );
        all_paths.insert(path_node.visited.len(), path_node.move_cost);
        
        let neighbors = path_node.calc_neighbors(&valves, minutes);
        if neighbors.is_empty() && max < path_node.pressure_level {
            println!("Found better path, pressure: {}, steps: {}", path_node.pressure_level, path_node.total_steps);
            path_node.visited
                .iter()
                .map(|valve| valve.0.borrow().name.to_string())
                .for_each(|name| print!(" {name}"));
            println!();
            max = max.max(path_node.pressure_level);
        }
        remaining_valves.extend(neighbors);
    }
    println!("Lowest move_cost for steps: {:?}.",
        all_paths
            .iter()
            .map(|(steps, move_cost)| format!("{steps}: {move_cost}; "))
            .collect::<String>());

    max
}

fn parse_into_valves(path: &str) -> HashMap<String, ValveTarget> {
    let input = fs::read_to_string(path).unwrap();
    let valves: HashMap<String, ValveTarget> = input.lines()
        .flat_map(Valve::from_str)
        .map(|valve| (valve.name.to_string(), ValveTarget(Rc::new(RefCell::new(valve)))))
        .collect();

    valves
        .iter()
        .for_each(|(_, valve)| valve.populate_targets(&valves));
    valves
}



#[cfg(test)]
mod tests {

    use crate::day_16::{calc_most_released_pressure, parse_into_valves, valve::ValveTarget};


    #[test]
    fn test_parse_into_valves() {
        let valves = parse_into_valves("./input/day_16.test.txt");
        assert_eq!(valves.len(), 10);
        assert_eq!(valves.get(&String::from("AA")).unwrap().0.borrow().flow_rate, 0);
        let binding = valves.get(&String::from("HH")).unwrap().0.borrow();
        let borrowed_targets: Vec<&ValveTarget> = binding.targets.iter().collect();
        assert_eq!(
            borrowed_targets,
            vec![valves.get(&String::from("GG")).unwrap()]);
    }
    #[test]
    fn test_part_1() {
        let most_released_pressure = calc_most_released_pressure("./input/day_16.test.txt", 30);
        assert_eq!(most_released_pressure, 1651);
    }
}
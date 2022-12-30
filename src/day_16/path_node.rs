use std::{collections::{HashSet, HashMap, BinaryHeap}, rc::Rc};

use super::valve::{ValveTarget, Valve};
#[derive(Debug)]
pub struct PathNode {
    pub move_cost: usize,
    pub pressure_level: usize,
    pub visited: Vec<ValveTarget>,
    pub total_steps: usize
}

impl Eq for PathNode {}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self
            .move_cost.cmp(&other.move_cost).reverse()
            .then_with(|| self.pressure_level.cmp(&other.pressure_level))
            .then_with(|| self.total_steps.cmp(&other.total_steps))
            .then_with(|| self.visited.cmp(&other.visited))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PathNode {
    fn calc_potential_total_pressure(&self, valves: &HashMap<String, ValveTarget>, total_steps: usize) -> usize {
        valves
            .values()
            .map(|valve| valve.get_flow_rate() * total_steps)
            .sum::<usize>() 
        - self.pressure_level
    }
    fn map_neighbor_data_to_path_node(&self, (valve, steps): (ValveTarget, usize), max_steps: usize, potential_pressure_level: usize) -> PathNode {
        let mut visited = self.visited.to_vec();
        visited.push(valve.clone());
        let released_pressure = valve.get_flow_rate() * (max_steps - self.total_steps - steps - 1);
        PathNode {
            pressure_level: self.pressure_level + released_pressure,
            total_steps: self.total_steps + steps + 1,
            visited,
            move_cost: potential_pressure_level - released_pressure
        }
    }
    pub fn calc_neighbors(&self, valves: &HashMap<String, ValveTarget>, max_steps: usize) -> BinaryHeap<PathNode> {
        // println!("Calc neighbors");
        let cur_valve = self.visited.last().unwrap();        
        let potential_pressure_level = self.calc_potential_total_pressure(valves, max_steps);
        let neighbors = cur_valve.calc_cost_to_neighbors(valves, &self.visited, max_steps - self.total_steps);
        neighbors
            .into_iter()
            .map(|neighbor| {
                self.map_neighbor_data_to_path_node(neighbor, max_steps, potential_pressure_level)
            })
            // .inspect(|node| println!(" {}, step cost: {}, steps: {}",
            //     node.most_recent().0.borrow().name,
            //     node.move_cost,
            //     node.total_steps,
            // ))
            .collect()
    }
}
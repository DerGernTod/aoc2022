use std::{rc::Rc, hash::Hash, str::FromStr, num::ParseIntError, string::ParseError, error, collections::{HashMap, BinaryHeap}, cell::RefCell, iter::{Filter, Map}};

#[derive(Debug)]
pub struct Valve {
    pub name: String,
    pub flow_rate: usize,
    pub targets: Vec<ValveTarget>,
    temp_targets: Vec<String>
}

impl Eq for Valve {}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Ord for Valve {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
            .then_with(|| self.flow_rate.cmp(&other.flow_rate))
    }
}

impl PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq)]
pub struct ValveTarget(pub Rc<RefCell<Valve>>);

impl Hash for ValveTarget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().name.hash(state);
    }
}

impl PartialEq for ValveTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for ValveTarget {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for ValveTarget {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Clone for ValveTarget {
    fn clone(&self) -> Self {
        ValveTarget(Rc::clone(&self.0))
    }
}

impl ValveTarget {
    pub fn get_flow_rate(&self) -> usize {
        return self.0.borrow().flow_rate;
    }
    pub fn populate_targets(&self, valves: &HashMap<String, ValveTarget>) {
        let mut mut_borrow = self.0.borrow_mut();
        mut_borrow.targets = mut_borrow.temp_targets
            .iter()
            .flat_map(|valve_name| valves.get(valve_name))
            .map(|valve| ValveTarget(Rc::clone(&valve.0)))
            .collect()
    }
    pub fn calc_cost_to_neighbors(&self, valves: &HashMap<String, ValveTarget>, visited_valves: &[ValveTarget], remaining_steps: usize) -> Vec<(ValveTarget, usize)> {
        let mut all_nodes: HashMap<String, usize> = valves.keys().map(|name| (name.to_string(), usize::MAX)).collect();
        let mut remaining: BinaryHeap<(usize, String)> = BinaryHeap::new();
        remaining.push((0, self.0.borrow().name.to_string()));
        let mut max_cost = 0;
        while let Some((cost, name)) = remaining.pop() {
            max_cost = max_cost.max(cost);
            if all_nodes.get(&name).unwrap() < &cost {
                continue;
            }
            remaining.extend(
                valves
                    .get(&name)
                    .unwrap().0
                    .borrow()
                    .targets
                    .iter()
                    .map(|target| (cost + 1, target.0.borrow().name.to_string())));
                    
            all_nodes.insert(name, cost);
        }
        all_nodes
            .into_iter()
            .map(|(name, steps)| {
                let valve = valves.get(&name).unwrap();
                (valve.clone(), steps)
            })
            .filter(|(valve, cost)| !visited_valves.contains(valve)
                && valve.get_flow_rate() > 0
                && cost < &remaining_steps)
            // .inspect(|(max_pressure, valve, cost)| 
            //     println!(" {}: {}, minimum of {} steps", valve.0.borrow().name, max_pressure, cost)
            // )
            .collect()
            
    }
}

impl FromStr for Valve {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned = s
            .replace("Valve ", "")
            .replace("has flow rate=", "")
            .replace(';', "")
            .replace(", ", ",");
        let mut spl = cleaned.split(' ');
        let name = String::from(spl.next().unwrap());
        let flow_rate = spl.next().unwrap().parse::<usize>()?;
        let temp_targets: Vec<String> = spl.nth(4).unwrap().split(',').map(String::from).collect();
        Ok(Valve { name, flow_rate, targets: vec![], temp_targets })
    }
}

impl Valve {
    
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Valve;

    #[test]
    fn test_from_str() {
        let valve = Valve::from_str("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB").unwrap();
        assert_eq!(valve.name, "AA");
        assert_eq!(valve.flow_rate, 0);
        assert_eq!(valve.temp_targets, vec![String::from("DD"), String::from("II"), String::from("BB")]);
        let valve = Valve::from_str("Valve HH has flow rate=22; tunnel leads to valve GG").unwrap();
        assert_eq!(valve.name, "HH");
        assert_eq!(valve.flow_rate, 22);
        assert_eq!(valve.temp_targets, vec![String::from("GG")]);
    }
}
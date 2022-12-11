use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub enum Operator {
    Multiply(Option<usize>),
    Add(Option<usize>)
}

pub type TestMonkey = (usize, usize, usize);

#[derive(PartialEq, Eq, Debug)]
pub struct Monkey {
    items: Vec<usize>,
    id: usize,
    pub handled_items: usize,
    test: TestMonkey,
    op: Operator
}

impl Monkey {
    pub fn new(id: usize, items: Vec<usize>, test: TestMonkey, op: Operator) -> Monkey {
        Monkey { id, items, handled_items: 0, test, op }
    }

    pub fn add_item(&mut self, item: usize) {
        self.items.push(item);
    }

    pub fn get_divisor(&self) -> usize {
        self.test.0
    }

    pub fn exec_round(&mut self, worry_divisor: Option<usize>) -> Vec<(usize, usize)> {
        let result: Vec<(usize, usize)> = self
            .items
            .iter()
            .map(|item| match self.op {
                Operator::Multiply(None) => item * item,
                Operator::Add(None) => item + item,
                Operator::Multiply(Some(x)) => item * x,
                Operator::Add(Some(x)) => item + x
            })
            .map(|item| {
                if let Some(divisor) = worry_divisor {
                    item % divisor
                } else {
                    f32::floor(item as f32 / 3.0) as usize
                }
            })
            .map(|item| {
                let (divisor, true_monkey, false_monkey) = self.test;
                let monkey_reveicer = if item % divisor == 0 {
                    true_monkey
                } else {
                    false_monkey
                };
                (monkey_reveicer, item)
            })
            .collect();
        self.handled_items += result.len();
        self.items = vec![];
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::day_11::monkey::Operator;

    use super::Monkey;

    #[test]
    fn test_exec_round() {
        let mut monkey = Monkey::new(
            0, 
            vec![79, 98], 
            (23, 2, 3),
            Operator::Multiply(Some(19))
        );
        let executions = monkey.exec_round(None);
        assert_eq!(executions, vec![(3, 500), (3, 620)]);
        assert_eq!(monkey.handled_items, 2);
    }
}
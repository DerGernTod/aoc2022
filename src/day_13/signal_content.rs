use std::{fmt::Display, cmp::Ordering};

#[derive(Debug, PartialEq, Eq)]
pub enum SignalContent {
    Number(usize),
    Block(Vec<SignalContent>)
}

impl SignalContent {
    pub fn generate_dividers() -> (Self, Self) {
        (
            SignalContent::Block(vec![SignalContent::Block(vec![SignalContent::Number(2)])]),
            SignalContent::Block(vec![SignalContent::Block(vec![SignalContent::Number(6)])])
        )
    }
}

impl PartialOrd for SignalContent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SignalContent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (SignalContent::Number(x), SignalContent::Number(y)) => x.cmp(y),
            (SignalContent::Number(n), SignalContent::Block(_)) => SignalContent::Block(vec![SignalContent::Number(*n)]).cmp(other),
            (SignalContent::Block(_), SignalContent::Number(n)) => self.cmp(&SignalContent::Block(vec![SignalContent::Number(*n)])),
            (SignalContent::Block(left), SignalContent::Block(right)) => {
                for i in 0..left.len().max(right.len()) {
                    let cur_left = left.get(i);
                    let cur_right = right.get(i);
                    if cur_left.is_none() {
                        return Ordering::Less;
                    } else if cur_right.is_none() {
                        return Ordering::Greater
                    }
                    let compare_entries = cur_left.unwrap().cmp(cur_right.unwrap());
                    match compare_entries {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => (),
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl Display for SignalContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalContent::Number(x) => 
                write!(f, "{}", x),
            SignalContent::Block(list) => {
                let str: Vec<String> = list
                    .iter()
                    .map(|entry| entry.to_string())
                    .collect();
                write!(f, "[{}]", str.join(","))
            },
        }
        
    }
}
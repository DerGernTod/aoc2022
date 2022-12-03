use std::collections::HashSet;

#[derive(Debug)]
pub struct Rucksack(pub Vec<char>);

pub fn char_to_value(c: &char) -> u32 {
    let char_val = *c as u32;
    if char_val > 96 {
        char_val - 96
    } else {
        char_val - 38
    }
}

impl Rucksack {
    pub fn shared_value(&self) -> u32 {
        let mut front = vec![];
        let mut back = vec![];
        for (i, ch) in self.0.iter().enumerate() {
            if i >= self.0.len() / 2 {
                back.push(*ch);
            } else {
                front.push(*ch);
            }
        }
        let duplicate_char = front
            .iter()
            .find(|ch| back.contains(*ch))
            .unwrap_or_else(|| panic!("Couldn't find a shared char between front and back: \n{:?}", self.0));
        char_to_value(duplicate_char)
    }
    pub fn find_matching_chars(&self, other: HashSet<char>) -> HashSet<char> {
        if other.is_empty() {
            HashSet::from_iter(self.0.iter().cloned())
        } else {
            self.0.clone().into_iter().filter(|ch| other.contains(ch)).collect()
        }
    }
}

impl FromIterator<char> for Rucksack {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{Rucksack, char_to_value};

    #[test]
    fn test_calc_shared_value() {
        println!("AaBb: {} {} {} {}", 'A' as u32, 'a' as u32, 'B' as u32, 'b' as u32);
        let rucksack: Rucksack = "VVaXXa".chars().collect();
        assert_eq!(rucksack.shared_value(), 1);
        let rucksack: Rucksack = "VVAXXA".chars().collect();
        assert_eq!(rucksack.shared_value(), 27);
    }

    #[test]
    fn test_char_to_value() {
        assert_eq!(char_to_value(&'A'), 27);
        assert_eq!(char_to_value(&'a'), 1);
    }

    #[test]
    fn test_find_matching_chars() {
        let rucksack: Rucksack = "VVaXX".chars().collect();
        assert_eq!(rucksack.find_matching_chars("yyayy".chars().collect()), HashSet::from(['a']));
        assert_eq!(rucksack.find_matching_chars("".chars().collect()), HashSet::from(['V','a','X']));
    }
}
#[derive(PartialEq, Eq, Debug)]
pub enum Shape {
    Rock,
    Paper,
    Scissors
}
impl Shape {
    pub fn from(ch: char) -> Shape {
        match ch {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Cannot create shape from char '{}'!", ch)
        }
    }
    pub fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Shape;

    #[test]
    fn test_score() {
        assert_eq!(Shape::Rock.score(), 1);
        assert_eq!(Shape::Paper.score(), 2);
        assert_eq!(Shape::Scissors.score(), 3);
    }

    #[test]
    fn test_from() {
        assert_eq!(Shape::Rock, Shape::from('A'));
        assert_eq!(Shape::Rock, Shape::from('X'));
        assert_eq!(Shape::Paper, Shape::from('B'));
        assert_eq!(Shape::Paper, Shape::from('Y'));
        assert_eq!(Shape::Scissors, Shape::from('C'));
        assert_eq!(Shape::Scissors, Shape::from('Z'));
    }

    #[test]
    #[should_panic(expected = "Cannot create shape from char ' '!")]
    fn test_from_panics() {
        Shape::from(' ');
    }
}
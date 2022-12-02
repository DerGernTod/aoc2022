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
    pub fn get_superior(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
    pub fn get_inferior(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
    pub fn score_as_outcome(&self, other: &Shape) -> u32 {
        match self {
            Shape::Rock => other.get_inferior().score(),
            Shape::Paper => other.score(),
            Shape::Scissors => other.get_superior().score(),
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

    #[test]
    fn test_superior() {
        assert_eq!(Shape::Rock.get_superior(), Shape::Paper);
        assert_eq!(Shape::Paper.get_superior(), Shape::Scissors);
        assert_eq!(Shape::Scissors.get_superior(), Shape::Rock);
    }

    #[test]
    fn test_inferior() {
        assert_eq!(Shape::Rock.get_inferior(), Shape::Scissors);
        assert_eq!(Shape::Paper.get_inferior(), Shape::Rock);
        assert_eq!(Shape::Scissors.get_inferior(), Shape::Paper);
    }

    #[test]
    fn test_score_as_outcome() {
        assert_eq!(Shape::Rock.score_as_outcome(&Shape::Rock), Shape::Scissors.score());
        assert_eq!(Shape::Paper.score_as_outcome(&Shape::Rock), Shape::Rock.score());
        assert_eq!(Shape::Scissors.score_as_outcome(&Shape::Rock), Shape::Paper.score());
    }
}
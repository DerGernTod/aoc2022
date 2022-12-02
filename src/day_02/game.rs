use super::shape::Shape;
#[derive(Eq, PartialEq, Debug)]
pub struct Game {
    opponent: Shape,
    me: Shape
}

impl Game {
    pub fn new(opponent: Shape, me: Shape) -> Game {
        Game {
            opponent,
            me
        }
    }
    pub fn score(&self) -> u32 {
        let game_score = match (&self.opponent, &self.me) {
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Paper, Shape::Rock) => 0,
            (Shape::Paper, Shape::Scissors) => 6,
            (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 0,
            _ => 3,
        };
        game_score + self.me.score()
    }
}

impl FromIterator<char> for Game {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Game::new(Shape::from(iter.next().unwrap()), Shape::from(iter.next().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use crate::day_02::{shape::Shape, game::Game};

    #[test]
    fn test_score() {
        assert_eq!(Game::new(Shape::Rock, Shape::Rock).score(), 4);
        assert_eq!(Game::new(Shape::Paper, Shape::Rock).score(), 1);
        assert_eq!(Game::new(Shape::Scissors, Shape::Rock).score(), 7);
    }

    #[test]
    fn test_from_iter() {
        let game: Game = "AX".chars().collect();
        assert_eq!(game, Game::new(Shape::Rock, Shape::Rock));
    }
}
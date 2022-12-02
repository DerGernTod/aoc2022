use std::fs;

use self::game::Game;

mod shape;
mod game;

pub fn day_02() {
    let games = read_to_games("./input/day_02.txt");
    println!("total score: {}", calc_score(&games));
    println!("total score as outcome: {}", calc_score_as_outcome(&games));
}

fn calc_score(games: &[Game]) -> u32 {
    games.iter().map(|game| game.score()).sum()
}

fn calc_score_as_outcome(games: &[Game]) -> u32 {
    games.iter().map(|game| game.score_as_outcome()).sum()
}

fn read_to_games(path: &str) -> Vec<Game> {
    let input = fs::read_to_string(path).unwrap();

    input
        .split('\n')
        .map(|line| line.chars().step_by(2).collect())
        .collect()
}


#[cfg(test)]
mod tests {
    use crate::day_02::{calc_score, calc_score_as_outcome};

    use super::read_to_games;

    #[test]
    fn test_part_1() {
        let games = read_to_games("./input/day_02.test.txt");
        assert_eq!(games.len(), 3);
        assert_eq!(calc_score(&games), 15);
    }

    #[test]
    fn test_part_2() {
        let games = read_to_games("./input/day_02.test.txt");
        assert_eq!(games.len(), 3);
        assert_eq!(calc_score_as_outcome(&games), 12);
    }
}
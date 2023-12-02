use std::str::FromStr;
pub struct Game {
    pub id: usize,
    pub games: Vec<GameRun>,
}

pub struct GameRun {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

enum Color {
    RED,
    GREEN,
    BLUE,
}

#[derive(Debug)]
pub struct ParseColorError;

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            _ => Err(ParseColorError),
        }
    }
}
#[derive(Debug)]
pub struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<_> = input.split(":").collect();
        let first = splitted
            .first()
            .ok_or_else(|| ParseGameError)?
            .split(" ")
            .collect::<Vec<_>>();
        let id = first.last().ok_or_else(|| ParseGameError)?;
        let id: usize = id.parse().map_err(|_| ParseGameError)?;
        let mut games = vec![];
        for run in splitted.last().ok_or_else(|| ParseGameError)?.split(";") {
            games.push(run.parse().map_err(|_| ParseGameError)?)
        }
        Ok(Game { id, games })
    }
}

#[derive(Debug)]
pub struct ParseGameRunError;

impl FromStr for GameRun {
    type Err = ParseGameRunError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut run = GameRun {
            red: 0,
            green: 0,
            blue: 0,
        };
        for n in input.trim().split(", ") {
            let parts: Vec<_> = n.split(" ").collect();
            let number = parts
                .first()
                .ok_or_else(|| ParseGameRunError)?
                .parse::<usize>()
                .map_err(|_| ParseGameRunError)?;
            let color = parts
                .last()
                .ok_or_else(|| ParseGameRunError)?
                .parse::<Color>()
                .map_err(|_| ParseGameRunError)?;
            match color {
                Color::RED => run.red = number,
                Color::GREEN => run.green = number,
                Color::BLUE => run.blue = number,
            }
        }
        Ok(run)
    }
}

impl Game {
    pub fn run_1_score(&self) -> usize {
        let max_red = self.games.iter().map(|run| run.red).max().unwrap_or(0);
        let max_green = self.games.iter().map(|run| run.green).max().unwrap_or(0);
        let max_blue = self.games.iter().map(|run| run.blue).max().unwrap_or(0);
        if max_red > 12 || max_green > 13 || max_blue > 14 {
            return 0;
        }

        self.id
    }
    pub fn min_power_score(&self) -> usize {
        let max_red = self.games.iter().map(|run| run.red).max().unwrap_or(0);
        let max_green = self.games.iter().map(|run| run.green).max().unwrap_or(0);
        let max_blue = self.games.iter().map(|run| run.blue).max().unwrap_or(0);
        max_red * max_green * max_blue
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_2_game_parse() {
        let game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red".parse::<Game>();
        assert!(game.is_ok());
        let game = game.unwrap();
        assert!(game.games.len() == 2)
    }
    #[test]
    fn test_2_game_run_parse() {
        let run = " 1 blue, 2 green".parse::<GameRun>();
        assert!(run.is_ok());
        let run = run.unwrap();
        assert_eq!(run.blue, 1);
        assert_eq!(run.green, 2);
        assert_eq!(run.red, 0);
    }
    #[test]
    fn test_2_min_pwor() {
        let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            .parse::<Game>();
        assert!(game.is_ok());
        let game = game.unwrap();
        assert_eq!(game.min_power_score(), 1560);
    }
}

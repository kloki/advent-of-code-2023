use card::{
    Game,
    GameVariant,
};
use toolkit::input::get_input;
mod card;

fn main() {
    let contents = get_input();

    let mut games = contents
        .trim()
        .split("\n")
        .map(|input| Game::new(input, GameVariant::Simple))
        .collect::<Result<Vec<Game>, _>>()
        .unwrap();

    games.sort();

    let day_1: usize = games
        .iter()
        .enumerate()
        .map(|(i, game)| (i + 1) * game.bid)
        .sum();
    println!("run1:{}", day_1);

    let mut games = contents
        .trim()
        .split("\n")
        .map(|input| Game::new(input, GameVariant::Joker))
        .collect::<Result<Vec<Game>, _>>()
        .unwrap();

    games.sort();

    let day_2: usize = games
        .iter()
        .enumerate()
        .map(|(i, game)| (i + 1) * game.bid)
        .sum();
    println!("run2:{}", day_2)
}

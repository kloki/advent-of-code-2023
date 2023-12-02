use std::{
    env,
    fs,
};

use game::Game;
mod game;

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    fs::read_to_string(path).expect("Can't read file")
}

fn main() {
    let contents = get_input();

    let games: Vec<Game> = contents
        .trim()
        .split("\n")
        .map(|game_line| game_line.parse().unwrap())
        .collect();
    let run_1_score: usize = games.iter().map(|game| game.run_1_score()).sum();
    println!("run 1: {}", run_1_score);
    let run_2_score: usize = games.iter().map(|game| game.min_power_score()).sum();
    println!("run 2: {}", run_2_score);
}

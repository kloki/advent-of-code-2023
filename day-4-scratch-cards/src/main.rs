use cards::{
    count_copies,
    Card,
};
use toolkit::input::get_input;
mod cards;

fn main() {
    let contents = get_input();
    let cards: Vec<Card> = contents
        .trim()
        .split("\n")
        .map(|c| c.parse().unwrap())
        .collect();
    let score: i32 = cards.iter().map(|c| c.score()).sum();

    println!("run 1: {}", score);
    let copies = count_copies(cards);
    println!("run 2: {}", copies);
}

use core::iter::zip;
use std::{
    cmp::Ordering,
    collections::HashMap,
};

#[derive(Debug, Clone, Copy)]
pub enum GameVariant {
    Simple,
    Joker,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(Debug)]
pub struct GameParseError;

impl Card {
    pub fn from_char(input: char, variant: GameVariant) -> Result<Self, GameParseError> {
        match input {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => match variant {
                GameVariant::Simple => Ok(Card::Jack),
                GameVariant::Joker => Ok(Card::Joker),
            },
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(GameParseError),
        }
    }
}
#[derive(Eq, Debug)]
pub struct Game {
    cards: Vec<Card>,
    pub bid: usize,
    hand: Hand,
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand.cmp(&other.hand) {
            Ordering::Equal => {
                for (card, other_card) in zip(&self.cards, &other.cards) {
                    match card.cmp(&other_card) {
                        Ordering::Equal => {}
                        o => return o,
                    };
                }
                Ordering::Equal
            }
            o => o,
        }
    }
}
impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Game {
    fn determine_hand(cards: &Vec<Card>) -> Hand {
        let mut counts: HashMap<Card, usize> = HashMap::new();
        for c in cards {
            match counts.get_mut(c) {
                Some(s) => {
                    *s += 1;
                }
                None => {
                    counts.insert(*c, 1);
                }
            }
        }
        let joker_count = *counts.get(&Card::Joker).unwrap_or(&0);
        counts.remove(&Card::Joker);
        let counts: Vec<usize> = counts.values().map(|x| *x).collect();

        if counts.contains(&5) || joker_count == 5 || counts.contains(&(5 - joker_count)) {
            return Hand::FiveOfAKind;
        }

        if counts.contains(&4) || joker_count == 4 || counts.contains(&(4 - joker_count)) {
            return Hand::FourOfAKind;
        }

        if counts.contains(&3) && counts.contains(&2)
            || (counts.clone().into_iter().filter(|x| *x == 2).count() == 2 && joker_count == 1)
        {
            return Hand::FullHouse;
        }
        if counts.contains(&3) || counts.contains(&(3 - joker_count)) {
            return Hand::ThreeOfAKind;
        }

        if (counts.clone().into_iter().filter(|x| *x == 2).count() == 2)
            || counts.contains(&2) && joker_count != 0
        {
            return Hand::TwoPair;
        }
        if counts.contains(&2) || joker_count != 0 {
            return Hand::OnePair;
        }

        Hand::HighCard
    }
    pub fn new(input: &str, variant: GameVariant) -> Result<Self, GameParseError> {
        let (cards, bid) = input.split_once(" ").ok_or(GameParseError)?;
        let bid: usize = bid.parse().map_err(|_| GameParseError)?;
        let cards: Vec<Card> = cards
            .chars()
            .map(|c| Card::from_char(c, variant))
            .collect::<Result<Vec<_>, _>>()?;
        let hand = Game::determine_hand(&cards);
        Ok(Game { cards, bid, hand })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let game: Game = Game::new("AAAA2 12", GameVariant::Simple).unwrap();
        assert_eq!(game.bid, 12);
        assert_eq!(game.hand, Hand::FourOfAKind);
    }

    #[test]
    fn test_sorting() {
        let mut games: Vec<Game> = vec![
            Game::new("AAA22 2", GameVariant::Simple).unwrap(),
            Game::new("AAAA2 3", GameVariant::Simple).unwrap(),
            Game::new("AAAA4 4", GameVariant::Simple).unwrap(),
            Game::new("AAJJ2 1", GameVariant::Simple).unwrap(),
        ];
        games.sort();

        dbg!(&games);
        assert_eq! {games[0].bid,1}
        assert_eq! {games[1].bid,2}
        assert_eq! {games[2].bid,3}
        assert_eq! {games[3].bid,4}
    }
    #[test]
    fn test_parse_joker() {
        let game: Game = Game::new("AAJA2 12", GameVariant::Joker).unwrap();
        assert_eq!(game.bid, 12);
        assert_eq!(game.hand, Hand::FourOfAKind);
    }
    #[test]
    fn test_parse_joker_upgrade() {
        let game: Game = Game::new("AAJ33 12", GameVariant::Joker).unwrap();
        assert_eq!(game.hand, Hand::FullHouse);
        let game: Game = Game::new("AAAJ3 12", GameVariant::Joker).unwrap();
        assert_eq!(game.hand, Hand::FourOfAKind);
    }

    #[test]
    fn test_parse_joker_sort() {
        let mut games: Vec<Game> = vec![
            Game::new("32T3K 1", GameVariant::Joker).unwrap(),
            Game::new("T55J5 3", GameVariant::Joker).unwrap(),
            Game::new("KK677 2", GameVariant::Joker).unwrap(),
            Game::new("KTJJT 5", GameVariant::Joker).unwrap(),
            Game::new("QQQJA 4", GameVariant::Joker).unwrap(),
        ];
        games.sort();

        assert_eq! {games[0].bid,1}
        assert_eq! {games[1].bid,2}
        assert_eq! {games[2].bid,3}
        assert_eq! {games[3].bid,4}
        assert_eq! {games[4].bid,5}
    }
}

use std::str::FromStr;
#[allow(dead_code)]
pub struct Card {
    id: usize,
    numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
    quantity: usize,
}

impl Card {
    pub fn winnings(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .collect::<Vec<_>>()
            .len()
    }
    pub fn score(&self) -> i32 {
        let winnings = self.winnings();
        if winnings == 0 {
            return 0;
        }
        2i32.pow(winnings as u32 - 1)
    }
}

#[derive(Debug)]
pub struct ParseCardError;
impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (id, numbers) = input.split_once(":").ok_or(ParseCardError)?;
        let id = id
            .split(" ")
            .filter(|x| *x != "")
            .last()
            .ok_or(ParseCardError)?;
        let id: usize = id.parse().map_err(|_| ParseCardError)?;
        let (numbers, winning_numbers) = numbers.split_once("|").ok_or(ParseCardError)?;
        let numbers = numbers
            .trim()
            .split(" ")
            .filter(|x| *x != "")
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| ParseCardError)?;
        let winning_numbers = winning_numbers
            .trim()
            .split(" ")
            .filter(|x| *x != "")
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| ParseCardError)?;

        Ok(Card {
            id,
            numbers,
            winning_numbers,
            quantity: 1,
        })
    }
}

pub fn count_copies(mut cards: Vec<Card>) -> usize {
    let winnings_list: Vec<usize> = cards.iter().map(|c| c.winnings()).collect();
    for (i, winnings) in winnings_list.iter().enumerate() {
        for ii in (i + 1)..(i + 1 + winnings) {
            cards[ii].quantity += cards[i].quantity;
        }
    }
    cards.iter().map(|c| c.quantity).sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_4_scratch_card_parse() {
        let card = "Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse::<Card>();
        assert!(card.is_ok());
        let card = card.unwrap();
        assert!(card.numbers.len() == 5);
        assert!(card.winning_numbers.len() == 8);
    }
    #[test]
    fn test_4_test_score() {
        let card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse::<Card>();
        assert!(card.is_ok());
        let card = card.unwrap();
        assert!(card.score() == 8);
    }

    #[test]
    fn test_4_test_breaking() {
        let card = "Card  12: 95 46  8 27 41 34 82  4 84 59 | 52 39  7  3 54 57 29  1 21 89 75 33 14 94 36 15 60 40 16 80 35 83  9  5 87".parse::<Card>();
        assert!(card.is_ok());
        let card = card.unwrap();
        assert!(card.score() == 0);
    }
    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_4_test_scratch_cards_copies() {
        let cards: Vec<Card> = TEST_INPUT
            .trim()
            .split("\n")
            .map(|c| c.parse().unwrap())
            .collect();
        assert_eq!(count_copies(cards), 30)
    }
}

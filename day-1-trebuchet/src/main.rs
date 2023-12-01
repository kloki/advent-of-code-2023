use std::{env, fs};

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    fs::read_to_string(path).expect("Can't read file")
}

fn parse_run_1(input: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = vec![];
    for n in input.chars() {
        if let Some(number) = n.to_digit(10) {
            numbers.push(number)
        }
    }
    numbers
}

fn check_word(input: &str, word: &str, i: usize) -> bool {
    input[i..input.len().min(i + word.len())] == *word
}

fn parse_run_2(input: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = vec![];
    for (i, n) in input.chars().enumerate() {
        if let Some(number) = n.to_digit(10) {
            numbers.push(number);
        } else {
            for number in 0..10 {
                if check_word(input, NUMBERS[number], i) {
                    numbers.push(number as u32);
                    continue;
                }
            }
        }
    }
    numbers
}
fn score(numbers: Vec<u32>) -> u32 {
    10 * numbers[0] + numbers[numbers.len() - 1]
}

fn main() {
    let contents = get_input();

    let score_run_1: u32 = contents
        .trim()
        .split("\n")
        .map(|input| parse_run_1(input))
        .map(|numbers| score(numbers))
        .sum();
    println!("run 1:{}", score_run_1);
    let score_run_2: u32 = contents
        .trim()
        .split("\n")
        .map(|input| parse_run_2(input))
        .map(|numbers| score(numbers))
        .sum();
    println!("run 2:{}", score_run_2);
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1_trebuchet_run_1_simple() {
        assert_eq!(score(parse_run_1("1abc2")), 12)
    }
    #[test]
    fn test_1_trebuchet_run_1_one_digit() {
        assert_eq!(score(parse_run_1("7ash")), 77)
    }

    #[test]
    fn test_1_trebuchet_run_2_one_written() {
        assert_eq!(score(parse_run_2("7asones")), 71)
    }

    #[test]
    fn test_1_trebuchet_run_2_one_overlap() {
        assert_eq!(score(parse_run_2("7asheightwo")), 72)
    }
}

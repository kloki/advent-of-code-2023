use std::iter::zip;

use toolkit::input::get_input;
#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    pub fn new(time: usize, distance: usize) -> Self {
        Race { time, distance }
    }

    pub fn win_options(&self) -> usize {
        (1..(self.time + 1))
            .map(|x| (self.time - x) * x)
            .filter(|x| *x > self.distance)
            .collect::<Vec<_>>()
            .len()
    }
}
#[derive(Debug)]
struct ParseLineError;
fn parse_line_1(input: &str) -> Result<Vec<usize>, ParseLineError> {
    let (_, numbers) = input.split_once(":").ok_or(ParseLineError)?;
    let numbers = numbers
        .trim()
        .split(" ")
        .filter(|x| *x != "")
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|_| ParseLineError)?;
    Ok(numbers)
}

fn parse_line_2(input: &str) -> Result<usize, ParseLineError> {
    let (_, numbers) = input.split_once(":").ok_or(ParseLineError)?;
    let number = numbers
        .trim()
        .split(" ")
        .filter(|x| *x != "")
        .collect::<String>()
        .parse::<usize>()
        .map_err(|_| ParseLineError)?;
    Ok(number)
}
fn main() {
    let contents = get_input();
    let parsed = contents
        .trim()
        .split("\n")
        .map(|x| parse_line_1(x))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let races: Vec<Race> = zip(&parsed[0], &parsed[1])
        .map(|(time, distance)| Race::new(*time, *distance))
        .collect();

    let product: usize = races.iter().map(|x| x.win_options()).product();
    println!("run 1:{}", product);

    let parsed = contents
        .trim()
        .split("\n")
        .map(|x| parse_line_2(x))
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();
    let race = Race::new(parsed[0], parsed[1]);

    println!("run 2:{}", race.win_options());
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_number_of_wins() {
        let race = Race::new(7, 9);
        assert_eq!(race.win_options(), 4)
    }
}

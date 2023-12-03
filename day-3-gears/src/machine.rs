use std::collections::HashMap;

enum Tile {
    Number(u32),
    Symbol,
    None,
}

impl Tile {
    fn build(input: char) -> Tile {
        if input == '.' {
            return Tile::None;
        }
        if let Some(s) = input.to_digit(10) {
            return Tile::Number(s);
        }
        Tile::Symbol
    }

    fn is_symbol(input: char) -> bool {
        match Tile::build(input) {
            Tile::Symbol => true,
            _ => false,
        }
    }
}
#[derive(Debug)]
pub struct Part {
    pub number: u32,
    pub locations: Vec<(usize, usize)>,
}

impl Part {
    fn new() -> Part {
        Part {
            number: 0,
            locations: vec![],
        }
    }
    fn append(&mut self, other: u32, x: usize, y: usize) {
        self.locations.push((x, y));
        self.number *= 10;
        self.number += other;
    }

    fn active(&self) -> bool {
        self.number != 0
    }

    fn is_touching(&self, x: usize, y: usize) -> bool {
        self.locations.contains(&(x, y))
    }
}

pub fn parse_machine(input: String) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .map(|row| row.chars().collect())
        .collect()
}

pub fn is_near_symbol(machine: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x != 0 && Tile::is_symbol(machine[x - 1][y]) {
        return true;
    }
    if Tile::is_symbol(machine[x][y]) {
        return true;
    }
    if x < machine.len() - 1 && Tile::is_symbol(machine[x + 1][y]) {
        return true;
    }
    false
}

pub fn get_parts(machine: &Vec<Vec<char>>) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();
    let mut current = Part::new();
    let mut near_symbol = false;
    for (x, row) in machine.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if !near_symbol {
                near_symbol = is_near_symbol(&machine, x, y);
            }
            match Tile::build(*c) {
                Tile::Number(s) => current.append(s, x, y),
                _ => {
                    if current.active() && near_symbol {
                        parts.push(current);
                    }
                    current = Part::new();
                    near_symbol = is_near_symbol(&machine, x, y);
                }
            }
        }
        if current.active() && near_symbol {
            parts.push(current);
        }
        current = Part::new();
        near_symbol = false;
    }
    parts.sort_unstable_by_key(|e| e.number);
    parts
}

fn get_touching_part(parts: &Vec<Part>, x: usize, y: usize) -> Option<((usize, usize), u32)> {
    for part in parts {
        if part.is_touching(x, y) {
            return Some((part.locations[0], part.number));
        }
    }
    None
}

fn get_touching_parts(
    parts: &Vec<Part>,
    x: usize,
    y: usize,
    x_max: usize,
    y_max: usize,
) -> Vec<u32> {
    let mut gears_set: HashMap<(usize, usize), u32> = HashMap::new();

    if x > 0 && y > 0 {
        if let Some(n) = get_touching_part(&parts, x - 1, y - 1) {
            gears_set.insert(n.0, n.1);
        }
    }

    if x > 0 {
        if let Some(n) = get_touching_part(&parts, x - 1, y) {
            gears_set.insert(n.0, n.1);
        }
    }

    if x > 0 && y < y_max {
        if let Some(n) = get_touching_part(&parts, x - 1, y + 1) {
            gears_set.insert(n.0, n.1);
        }
    }

    if y > 0 {
        if let Some(n) = get_touching_part(&parts, x, y - 1) {
            gears_set.insert(n.0, n.1);
        }
    }

    if y < y_max {
        if let Some(n) = get_touching_part(&parts, x, y + 1) {
            gears_set.insert(n.0, n.1);
        }
    }
    if x < x_max && y > 0 {
        if let Some(n) = get_touching_part(&parts, x + 1, y - 1) {
            gears_set.insert(n.0, n.1);
        }
    }
    if x < x_max {
        if let Some(n) = get_touching_part(&parts, x + 1, y) {
            gears_set.insert(n.0, n.1);
        }
    }

    if x < x_max && y < y_max {
        if let Some(n) = get_touching_part(&parts, x + 1, y + 1) {
            gears_set.insert(n.0, n.1);
        }
    }
    gears_set.values().map(|x| *x).collect()
}

pub fn get_gears(machine: &Vec<Vec<char>>, parts: &Vec<Part>) -> Vec<u32> {
    let mut gears: Vec<u32> = Vec::new();
    let x_max = machine.len() - 1;
    let y_max = machine[0].len() - 1;
    for (x, row) in machine.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == '*' {
                let gear_set = get_touching_parts(&parts, x, y, x_max, y_max);

                if gear_set.len() == 2 {
                    gears.push(gear_set[0] * gear_set[1]);
                }
            }
        }
    }
    gears
}
#[cfg(test)]
mod tests {

    use super::*;
    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const TEST_TWIN: &str = "467..114..
...*......
..35..633.
......#...
617*617...
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_3_gears_parts() {
        let machine = parse_machine(TEST_INPUT.to_string());
        let parts = get_parts(&machine);

        assert_eq!(parts.len(), 8);
        assert_eq!(parts.iter().map(|x| x.number).sum::<u32>(), 4361);
    }

    #[test]
    fn test_3_gears_gear_ratios() {
        let machine = parse_machine(TEST_INPUT.to_string());
        let parts = get_parts(&machine);
        let gears = get_gears(&machine, &parts);

        assert_eq!(gears.len(), 2);
        assert_eq!(gears.iter().sum::<u32>(), 467835);
    }

    #[test]
    fn test_3_gears_gear_ratios_twins() {
        let machine = parse_machine(TEST_TWIN.to_string());
        let parts = get_parts(&machine);
        let gears = get_gears(&machine, &parts);

        assert_eq!(gears.len(), 3);
    }
}

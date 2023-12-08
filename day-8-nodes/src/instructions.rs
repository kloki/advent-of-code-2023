use std::str::FromStr;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn from_char(input: char) -> Result<Self, ParseInstructionsError> {
        match input {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(ParseInstructionsError),
        }
    }
}

#[derive(Clone)]
pub struct Instructions {
    instructions: Vec<Instruction>,
    index: usize,
}

impl Instructions {
    pub fn reset(&mut self) {
        self.index = 0
    }
}

impl Iterator for Instructions {
    type Item = Instruction;
    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.index;
        self.index += 1;
        if self.index == self.instructions.len() {
            self.index = 0
        }
        Some(self.instructions[curr])
    }
}
#[derive(Debug)]
pub struct ParseInstructionsError;

impl FromStr for Instructions {
    type Err = ParseInstructionsError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let instructions = input
            .trim()
            .chars()
            .map(|c| Instruction::from_char(c))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Instructions {
            instructions,
            index: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let ins: Instructions = "LRLRLRRR".parse().unwrap();
        assert_eq!(ins.instructions.len(), 8)
    }
    #[test]
    fn test_iter() {
        let mut ins: Instructions = "LRL".parse().unwrap();
        assert_eq!(ins.next().unwrap(), Instruction::Left);
        assert_eq!(ins.next().unwrap(), Instruction::Right);
        assert_eq!(ins.next().unwrap(), Instruction::Left);
        assert_eq!(ins.next().unwrap(), Instruction::Left);
    }
}

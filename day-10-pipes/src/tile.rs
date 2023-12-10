#[derive(Debug)]
pub enum TileType {
    Start,
    Empty,
    Vertical,
    Horizontal,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

#[derive(Debug)]
pub struct TileParseError;
impl TileType {
    pub fn from_char(c: char) -> Result<TileType, TileParseError> {
        match c {
            ' ' => Ok(TileType::Empty),
            'S' => Ok(TileType::Start),
            '─' => Ok(TileType::Horizontal),
            '│' => Ok(TileType::Vertical),
            '┌' => Ok(TileType::RightDown),
            '┐' => Ok(TileType::LeftDown),
            '└' => Ok(TileType::RightUp),
            '┘' => Ok(TileType::LeftUp),
            _ => Err(TileParseError),
        }
    }

    pub fn left_access(&self) -> bool {
        match self {
            TileType::Horizontal | TileType::LeftDown | TileType::LeftUp => true,
            _ => false,
        }
    }
    pub fn right_access(&self) -> bool {
        match self {
            TileType::Horizontal | TileType::RightDown | TileType::RightUp => true,
            _ => false,
        }
    }
    pub fn up_access(&self) -> bool {
        match self {
            TileType::Vertical | TileType::LeftUp | TileType::RightUp => true,
            _ => false,
        }
    }
    pub fn down_access(&self) -> bool {
        match self {
            TileType::Vertical | TileType::LeftDown | TileType::RightDown => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    tt: TileType,
    distance: Option<usize>,
}

impl Tile {
    pub fn from_char(c: char) -> Result<Tile, TileParseError> {
        let mut distance = None;
        let tt = TileType::from_char(c)?;
        if let TileType::Start = tt {
            distance = Some(0);
        }
        Ok(Tile { tt, distance })
    }
}

pub fn parse_board(board: &String) -> Vec<Vec<Tile>> {
    board
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| Tile::from_char(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "  ┌┐
 ┌┘│
S┘ └┐
│┌──┘
└┘   ";

    #[test]
    fn test_parse() {
        parse_board(&TEST_INPUT.to_string());
    }
}

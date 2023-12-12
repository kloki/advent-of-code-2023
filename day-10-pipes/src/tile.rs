use toolkit::grid::Grid;
#[derive(Debug, Clone)]
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
            '.' => Ok(TileType::Empty),
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
            TileType::Start | TileType::Horizontal | TileType::LeftDown | TileType::LeftUp => true,
            _ => false,
        }
    }
    pub fn right_access(&self) -> bool {
        match self {
            TileType::Start | TileType::Horizontal | TileType::RightDown | TileType::RightUp => {
                true
            }
            _ => false,
        }
    }
    pub fn up_access(&self) -> bool {
        match self {
            TileType::Start | TileType::Vertical | TileType::LeftUp | TileType::RightUp => true,
            _ => false,
        }
    }
    pub fn down_access(&self) -> bool {
        match self {
            TileType::Start | TileType::Vertical | TileType::LeftDown | TileType::RightDown => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub tt: TileType,
    pub distance: Option<usize>,
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
    pub fn is_start(&self) -> bool {
        match self.tt {
            TileType::Start => true,
            _ => false,
        }
    }
    pub fn on_loop(&self) -> bool {
        self.distance.is_some()
    }
}

pub fn parse_board(board: &String) -> Grid<Tile> {
    let tiles = board
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| Tile::from_char(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    tiles.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "..┌┐.
.┌┘│.
S┘.└┐
│┌──┘
└┘...";

    #[test]
    fn test_parse() {
        parse_board(&TEST_INPUT.to_string());
    }
}

use std::collections::VecDeque;

use tile::{
    parse_board,
    Tile,
};
use toolkit::{
    grid::Grid,
    input::get_input_name,
};
mod tile;

fn build_loop(board: &mut Grid<Tile>) {
    let (start, _) = board
        .into_iter()
        .filter(|(_, t)| t.is_start())
        .collect::<Vec<_>>()[0];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(start);
    while !queue.is_empty() {
        let coor = queue.pop_front().unwrap();
        let current_tile = board.get(coor).unwrap().to_owned();
        let distance = Some(current_tile.distance.unwrap() + 1);
        // left
        if current_tile.tt.left_access() && coor.1 != 0 {
            let new_coor = (coor.0, coor.1 - 1);
            if let Some(new_tile) = board.get_mut(new_coor) {
                if new_tile.distance.is_none() && new_tile.tt.right_access() {
                    new_tile.distance = distance;
                    queue.push_back(new_coor);
                }
            }
        }

        if current_tile.tt.right_access() {
            let new_coor = (coor.0, coor.1 + 1);
            if let Some(new_tile) = board.get_mut(new_coor) {
                if new_tile.distance.is_none() && new_tile.tt.left_access() {
                    new_tile.distance = distance;
                    queue.push_back(new_coor);
                }
            }
        }

        if current_tile.tt.up_access() && coor.0 != 0 {
            let new_coor = (coor.0 - 1, coor.1);
            if let Some(new_tile) = board.get_mut(new_coor) {
                if new_tile.distance.is_none() && new_tile.tt.down_access() {
                    new_tile.distance = distance;
                    queue.push_back(new_coor);
                }
            }
        }

        if current_tile.tt.down_access() {
            let new_coor = (coor.0 + 1, coor.1);
            if let Some(new_tile) = board.get_mut(new_coor) {
                if new_tile.distance.is_none() && new_tile.tt.up_access() {
                    new_tile.distance = distance;
                    queue.push_back(new_coor);
                }
            }
        }
    }
}

fn enclosed_area(board: &Grid<Tile>) -> usize {
    let mut counter = 0;
    for (coor, tile) in board.into_iter() {
        if !tile.on_loop() {
            let mut jumps = 0;
            let mut up_in = false;
            let mut down_in = false;
            for i in 0..coor.1 {
                let tile = &board.grid[coor.0][i];
                if tile.on_loop() {
                    match tile.tt {
                        tile::TileType::Vertical => jumps += 1,
                        tile::TileType::RightUp => up_in = true,
                        tile::TileType::RightDown => down_in = true,
                        tile::TileType::LeftDown => {
                            if up_in {
                                jumps += 1
                            }
                            up_in = false;
                            down_in = false;
                        }
                        tile::TileType::LeftUp => {
                            if down_in {
                                jumps += 1
                            }
                            up_in = false;
                            down_in = false;
                        }

                        _ => {}
                    }
                }
            }
            if (jumps % 2) != 0 {
                counter += 1;
            }
        }
    }
    counter
}
fn main() {
    let contents = get_input_name("./input/input_readable.txt".to_owned());
    let mut board: Grid<Tile> = parse_board(&contents);
    build_loop(&mut board);

    let max_distance = board
        .into_iter()
        .filter(|(_, t)| t.distance.is_some())
        .map(|(_, t)| t.distance.unwrap())
        .max()
        .unwrap();

    println!("run 1: {}", max_distance);
    println!("run 2: {}", enclosed_area(&board));
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "..┌┐.
.┌┘│.
S┘.└┐
│┌──┘
└┘...";

    const TEST_INPUT2: &str = "...........
.S───────┐.
.│┌─────┐│.
.││.....││.
.││.....││.
.│└─┐.┌─┘│.
.│..│.│..│.
.└──┘.└──┘.
...........
";
    #[test]
    fn test_build_loop() {
        let mut board = parse_board(&TEST_INPUT.to_string());
        build_loop(&mut board);
        let max_distance = board
            .into_iter()
            .filter(|(_, t)| t.distance.is_some())
            .map(|(_, t)| t.distance.unwrap())
            .max()
            .unwrap();
        assert_eq!(max_distance, 8)
    }

    #[test]
    fn test_enclosed_area() {
        let mut board = parse_board(&TEST_INPUT2.to_string());
        build_loop(&mut board);

        assert_eq!(enclosed_area(&board), 4)
    }
}

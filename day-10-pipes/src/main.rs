use tile::parse_board;
use toolkit::input::get_input_name;
mod tile;

fn main() {
    let contents = get_input_name("./input/input_readable.txt".to_owned());
    let board = parse_board(&contents);
    dbg!(board);
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_0_cookie_cutter() {
        assert_eq!(1, 1)
    }
}

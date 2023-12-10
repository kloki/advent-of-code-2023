use toolkit::input::get_input_name;

fn main() {
    let contents = get_input_name("./input/input_readable.txt".to_owned());
    println!("{}", contents);
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_0_cookie_cutter() {
        assert_eq!(1, 1)
    }
}

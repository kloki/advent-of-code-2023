use std::{
    env,
    fs,
};

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    fs::read_to_string(path).expect("Can't read file")
}

fn main() {
    let contents = get_input();
    println!("{}", contents);
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_0_cooie_cutter() {
        assert_eq!(1, 1)
    }
}

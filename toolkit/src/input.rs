use std::{
    env,
    fs,
};

pub fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    fs::read_to_string(path).expect("Can't read file")
}

pub fn get_input_name(fallback: String) -> String {
    let args: Vec<String> = env::args().collect();
    let path = &args.get(1).unwrap_or(&fallback);
    fs::read_to_string(path).expect("Can't read file")
}

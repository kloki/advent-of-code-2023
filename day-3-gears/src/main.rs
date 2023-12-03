use std::{
    env,
    fs,
};

use machine::{
    get_gears,
    get_parts,
    parse_machine,
};
mod machine;

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    fs::read_to_string(path).expect("Can't read file")
}

fn main() {
    let contents = get_input();
    let machine = parse_machine(contents);
    let parts = get_parts(&machine);

    println!("run 1: {}", parts.iter().map(|x| x.number).sum::<u32>());
    let gears = get_gears(&machine, &parts);
    println!("run 2: {}", gears.iter().sum::<u32>());
}

use machine::{
    get_gears,
    get_parts,
    parse_machine,
};
mod machine;
use toolkit::input::get_input;

fn main() {
    let contents = get_input();
    let machine = parse_machine(contents);
    let parts = get_parts(&machine);

    println!("run 1: {}", parts.iter().map(|x| x.number).sum::<u32>());
    let gears = get_gears(&machine, &parts);
    println!("run 2: {}", gears.iter().sum::<u32>());
}

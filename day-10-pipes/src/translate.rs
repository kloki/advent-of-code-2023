use toolkit::input::get_input;

fn main() {
    let contents = get_input();
    let contents = contents.replace("-", "─");
    let contents = contents.replace("|", "│");
    let contents = contents.replace("F", "┌");
    let contents = contents.replace("7", "┐");
    let contents = contents.replace("L", "└");
    let contents = contents.replace("J", "┘");
    println!("{}", contents);
}

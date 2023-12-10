use toolkit::input::get_input;

fn main() {
    let contents = get_input();
    let lines: Vec<Vec<isize>> = contents
        .trim()
        .split("\n")
        .map(|l| {
            l.trim()
                .split(" ")
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    dbg!(lines);
}

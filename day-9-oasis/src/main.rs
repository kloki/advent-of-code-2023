use report::{
    predict_next,
    predict_previous,
};
use toolkit::input::get_input;
mod report;

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

    let run_1: isize = lines.iter().map(|x| predict_next(x)).sum();
    println!("run 1:{}", run_1);
    let run_2: isize = lines.iter().map(|x| predict_previous(x)).sum();
    println!("run 2:{}", run_2);
}

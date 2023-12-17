use galaxy::{
    apply_spaces,
    build_universe,
    get_distances,
    get_galaxies,
    get_spaces,
};
use toolkit::input::get_input;

mod galaxy;

fn main() {
    let contents = get_input();
    let universe = build_universe(contents).unwrap();
    let galaxies = get_galaxies(&universe);

    let mut distances = get_distances(&galaxies);
    let (horizontal, vertical) = get_spaces(&universe);
    apply_spaces(1, &mut distances, &horizontal, &vertical);

    let run_1 = distances.iter().map(|x| x.distance).sum::<usize>();
    println!("run 1:{}", run_1);

    let mut distances = get_distances(&galaxies);
    apply_spaces(999999, &mut distances, &horizontal, &vertical);

    let run_2 = distances.iter().map(|x| x.distance).sum::<usize>();
    println!("run 2:{}", run_2);
}

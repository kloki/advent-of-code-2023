use seeds::parse_input;
use toolkit::input::get_input;
mod seeds;
fn main() {
    let contents = get_input();

    let (seeds, almanac) = parse_input(&contents).unwrap();

    let mininum_location = seeds
        .iter()
        .map(|seed| almanac.process(*seed))
        .min()
        .unwrap();
    println!("run 1:{}", mininum_location);

    let mut mininum_locations: Vec<usize> = Vec::new();
    for i in 0..10 {
        println!("- step:{}/10", i + 1);
        let a = seeds[i * 2];
        let b = seeds[i * 2 + 1];
        let min = (a..(a + b))
            .map(|seed| almanac.process(seed))
            .min()
            .unwrap();
        mininum_locations.push(min);
    }

    println!("run 2:{}", mininum_locations.iter().min().unwrap());
}

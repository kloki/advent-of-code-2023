use toolkit::grid::Grid;
pub fn build_universe(input: String) -> Result<Grid<char>, &'static str> {
    input
        .trim()
        .split("\n")
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .try_into()
}

pub fn apply_spaces(
    size: usize,
    distances: &mut Vec<Distance>,
    horizontal: &Vec<usize>,
    vertical: &Vec<usize>,
) {
    for d in distances.iter_mut() {
        for vert in vertical {
            if d.a.1 <= *vert && *vert <= d.b.1 || d.b.1 <= *vert && *vert <= d.a.1 {
                d.distance += size;
            }
        }
    }

    for d in distances.iter_mut() {
        for hort in horizontal {
            if d.a.0 <= *hort && *hort <= d.b.0 || d.b.0 <= *hort && *hort <= d.a.0 {
                d.distance += size;
            }
        }
    }
}

pub fn get_spaces(universe: &Grid<char>) -> (Vec<usize>, Vec<usize>) {
    let mut horizontal: Vec<usize> = Vec::new();
    let mut vertical: Vec<usize> = Vec::new();

    for (i, row) in universe.grid.iter().enumerate() {
        if row.iter().all(|x| *x == '.') {
            horizontal.push(i);
        }
    }

    for i in 0..universe.grid[0].len() {
        if universe.grid.iter().all(|x| x[i] == '.') {
            vertical.push(i);
        }
    }
    (horizontal, vertical)
}

pub fn get_galaxies(universe: &Grid<char>) -> Vec<(usize, usize)> {
    universe
        .into_iter()
        .filter(|(_, x)| **x == '#')
        .map(|x| x.0)
        .collect()
}

pub fn get_distances(galaxies: &Vec<(usize, usize)>) -> Vec<Distance> {
    let mut distances: Vec<Distance> = Vec::new();
    for (i, a) in galaxies.iter().enumerate() {
        for b in &galaxies[i..galaxies.len()] {
            if a != b {
                distances.push(Distance::new(*a, *b))
            }
        }
    }
    distances
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Distance {
    a: (usize, usize),
    b: (usize, usize),
    pub distance: usize,
}

impl Distance {
    pub fn new(a: (usize, usize), b: (usize, usize)) -> Self {
        let distance =
            ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize;
        Self { a, b, distance }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    #[test]
    fn test_build() {
        build_universe(TEST_INPUT.to_string()).unwrap();
    }
    #[test]
    fn test_galaxies() {
        let universe = build_universe(TEST_INPUT.to_string()).unwrap();
        let galaxies = get_galaxies(&universe);
        assert_eq!(galaxies.len(), 9);
    }
    #[test]
    fn test_distances_run_1() {
        let universe = build_universe(TEST_INPUT.to_string()).unwrap();
        let galaxies = get_galaxies(&universe);
        let mut distances = get_distances(&galaxies);
        let (horizontal, vertical) = get_spaces(&universe);

        apply_spaces(1, &mut distances, &horizontal, &vertical);

        assert_eq!(distances.len(), 36);
        assert_eq!(distances.iter().map(|x| x.distance).sum::<usize>(), 374);
    }

    #[test]
    fn test_distances_run_2() {
        let universe = build_universe(TEST_INPUT.to_string()).unwrap();
        let galaxies = get_galaxies(&universe);
        let mut distances = get_distances(&galaxies);
        let (horizontal, vertical) = get_spaces(&universe);

        apply_spaces(9, &mut distances, &horizontal, &vertical);

        assert_eq!(distances.len(), 36);
        assert_eq!(distances.iter().map(|x| x.distance).sum::<usize>(), 1030);
    }
}

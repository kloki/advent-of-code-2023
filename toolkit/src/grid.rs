pub fn surrounding_coor(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut coor: Vec<(usize, usize)> = Vec::new();
    if x > 0 {
        if y > 0 {
            coor.push((x - 1, y - 1));
        }
        coor.push((x - 1, y));
        if y < y_max {
            coor.push((x - 1, y + 1));
        }
    }
    if y > 0 {
        coor.push((x, y - 1));
    }

    if y < y_max {
        coor.push((x, y + 1));
    }

    if x < x_max {
        if y > 0 {
            coor.push((x + 1, y - 1));
        }
        coor.push((x + 1, y));
        if y < y_max {
            coor.push((x + 1, y + 1));
        }
    }

    coor
}

#[derive(Clone, Debug)]
pub struct Grid<T: std::clone::Clone>(pub Vec<Vec<T>>);

impl<T: std::clone::Clone> Grid<T> {
    pub fn get(&self, coor: (usize, usize)) -> &T {
        &self.0[coor.1][coor.0]
    }

    pub fn iter(&self) -> std::vec::IntoIter<((usize, usize), T)> {
        self.clone().into_iter()
    }
}

impl<T: std::clone::Clone> IntoIterator for Grid<T> {
    type Item = ((usize, usize), T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut outer: Vec<((usize, usize), T)> = Vec::new();
        for (y, r) in self.0.iter().enumerate() {
            let next: Vec<((usize, usize), T)> = r
                .iter()
                .enumerate()
                .map(|(x, i)| ((x, y), i.clone()))
                .collect();
            outer.extend(next);
        }
        outer.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_iter() {
        let grid: Grid<usize> = Grid(vec![vec![0, 1], vec![2, 3]]);
        let items: Vec<_> = grid.iter().collect();
        assert_eq!(items.len(), 4);
        let two = grid
            .into_iter()
            .filter(|(_, x)| *x == 2)
            .collect::<Vec<_>>()[0];

        assert_eq!(two.1, 2);
        assert_eq!(two.0 .0, 0);
        assert_eq!(two.0 .1, 1);
    }
    #[test]
    fn test_grid_get() {
        let grid: Grid<usize> = Grid(vec![vec![0, 1], vec![2, 3]]);
        let two = grid.get((0, 1));

        assert_eq!(*two, 2);
    }
}

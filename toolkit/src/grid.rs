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
impl<T: std::clone::Clone> Grid<T> {
    pub fn get(&self, coor: (usize, usize)) -> &T {
        &self.0[coor.1][coor.0]
    }

    pub fn max_row(&self) -> usize {
        self.0.len() - 1
    }

    pub fn max_column(&self) -> usize {
        self.0[0].len() - 1
    }

    pub fn get_surrounding(&self, coor: (usize, usize)) -> Vec<((usize, usize), &T)> {
        let surrounding_coor = surrounding_coor(coor.0, coor.1, self.max_column(), self.max_row());
        surrounding_coor
            .iter()
            .map(|x| (*x, self.get(*x)))
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug)]
pub struct Grid<T: std::clone::Clone>(pub Vec<Vec<T>>);

pub struct GridBorrow<'a, T: std::clone::Clone> {
    grid: &'a Grid<T>,
    row: usize,
    col: usize,
}

impl<'a, T: Clone> IntoIterator for &'a Grid<T> {
    type Item = ((usize, usize), &'a T);
    type IntoIter = GridBorrow<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridBorrow {
            grid: self,
            row: 0,
            col: 0,
        }
    }
}

impl<'a, T: Clone> Iterator for GridBorrow<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row > self.grid.max_row() {
            return None;
        }
        let result = &self.grid.0[self.row][self.col];
        self.col += 1;
        if self.col > self.grid.max_column() {
            self.row += 1;
            self.col = 0;
        }
        Some(((self.col, self.row), result))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_iter() {
        let grid: Grid<usize> = Grid(vec![vec![0, 1], vec![2, 3]]);
        let items: Vec<_> = grid.into_iter().collect();

        assert_eq!(*items[2].1, 2);
    }
    #[test]
    fn test_grid_get() {
        let grid: Grid<usize> = Grid(vec![vec![0, 1], vec![2, 3]]);
        let two = grid.get((0, 1));

        assert_eq!(*two, 2);
    }
    #[test]
    fn test_grid_get_surounding() {
        let grid: Grid<usize> = Grid(vec![vec![0, 1], vec![2, 3]]);
        let tiles = grid.get_surrounding((0, 1));

        assert_eq!(tiles.len(), 3);
    }
}

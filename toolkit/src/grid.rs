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

pub fn neighbor_coor(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut coor: Vec<(usize, usize)> = Vec::new();
    if x > 0 {
        coor.push((x - 1, y));
    }
    if y > 0 {
        coor.push((x, y - 1));
    }

    if y < y_max {
        coor.push((x, y + 1));
    }

    if x < x_max {
        coor.push((x + 1, y));
    }

    coor
}
#[derive(Clone, Debug)]
pub struct Grid<T: Clone> {
    pub grid: Vec<Vec<T>>,
}

impl<T: std::clone::Clone> Grid<T> {
    pub fn new(rows: usize, columns: usize, placeholder: T) -> Grid<T> {
        Grid {
            grid: vec![vec![placeholder; columns]; rows],
        }
    }
    pub fn get(&self, coor: (usize, usize)) -> Option<&T> {
        if coor.0 > self.max_column() || coor.1 > self.max_row() {
            return None;
        }
        Some(&self.grid[coor.1][coor.0])
    }

    pub fn max_row(&self) -> usize {
        self.grid.len() - 1
    }

    pub fn max_column(&self) -> usize {
        self.grid[0].len() - 1
    }

    pub fn get_surrounding(&self, coor: (usize, usize)) -> Vec<((usize, usize), &T)> {
        let surrounding_coor = surrounding_coor(coor.0, coor.1, self.max_column(), self.max_row());
        surrounding_coor
            .iter()
            .map(|x| (*x, &self.grid[x.0][x.1]))
            .collect::<Vec<_>>()
    }
    pub fn get_neighbors(&self, coor: (usize, usize)) -> Vec<((usize, usize), &T)> {
        let neighbor_coor = neighbor_coor(coor.0, coor.1, self.max_column(), self.max_row());
        neighbor_coor
            .iter()
            .map(|x| (*x, &self.grid[x.0][x.1]))
            .collect::<Vec<_>>()
    }
}

impl<T: Clone> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = &'static str;

    fn try_from(input: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let width = input[0].len();
        if input.iter().filter(|x| x.len() != width).count() != 0 {
            Err("Not all rows have the same length.")
        } else {
            Ok(Grid { grid: input })
        }
    }
}
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
        let result = &self.grid.grid[self.row][self.col];
        let coor = (self.row, self.col);
        self.col += 1;
        if self.col > self.grid.max_column() {
            self.row += 1;
            self.col = 0;
        }
        Some((coor, result))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Tile(usize);

    #[test]
    fn test_grid_iter() {
        let grid: Grid<Tile> = vec![vec![Tile(0), Tile(1)], vec![Tile(2), Tile(3)]]
            .try_into()
            .unwrap();
        let items: Vec<_> = grid.into_iter().collect();

        assert_eq!(items[0], ((0, 0), &Tile(0)));
        assert_eq!(items[1], ((0, 1), &Tile(1)));
        assert_eq!(items[2], ((1, 0), &Tile(2)));
        assert_eq!(items[3], ((1, 1), &Tile(3)));
    }
    #[test]
    fn test_grid_get() {
        let grid: Grid<usize> = vec![vec![0, 1], vec![2, 3]].try_into().unwrap();
        let two = grid.get((0, 1));

        assert_eq!(*two.unwrap(), 2);

        let not_exist = grid.get((9, 9));
        assert!(not_exist.is_none())
    }
    #[test]
    fn test_grid_get_surounding() {
        let grid: Grid<usize> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 9]]
            .try_into()
            .unwrap();
        assert_eq!(grid.get_surrounding((0, 1)).len(), 5);
        assert_eq!(grid.get_surrounding((0, 0)).len(), 3);
        assert_eq!(grid.get_surrounding((1, 1)).len(), 8);
    }

    #[test]
    fn test_grid_get_neighbors() {
        let grid: Grid<usize> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 9]]
            .try_into()
            .unwrap();
        assert_eq!(grid.get_neighbors((0, 1)).len(), 3);
        assert_eq!(grid.get_neighbors((0, 0)).len(), 2);
        assert_eq!(grid.get_neighbors((1, 1)).len(), 4);
    }
}

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

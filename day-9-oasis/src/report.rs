pub fn predict_next_rec(input: &Vec<isize>) -> isize {
    let diff: Vec<isize> = input.windows(2).map(|x| x[1] - x[0]).collect();
    if diff.iter().all(|x| *x == 0) {
        return 0;
    }
    let last = *diff.last().unwrap();
    let next = predict_next_rec(&diff);
    next + last
}

pub fn predict_next(input: &Vec<isize>) -> isize {
    input.last().unwrap() + predict_next_rec(input)
}
pub fn predict_previous(input: &Vec<isize>) -> isize {
    let mut input = input.to_vec();
    input.reverse();
    predict_next(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future() {
        let input: Vec<isize> = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(predict_next(&input), 18);
        let input: Vec<isize> = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict_next(&input), 68)
    }

    #[test]
    fn test_past() {
        let input: Vec<isize> = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict_previous(&input), 5)
    }
}

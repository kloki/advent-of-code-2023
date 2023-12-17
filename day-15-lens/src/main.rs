use toolkit::input::get_input;

pub fn hash(input: &str) -> usize {
    input.chars().fold(0, |acc, x| {
        let mut val = acc;
        val += x as usize;
        val *= 17;
        val = val % 256;
        val
    })
}

fn main() {
    let contents = get_input();

    let run_1: usize = contents.trim().split(",").map(|seq| hash(seq)).sum();
    println!("run 1:{}", run_1);
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_hash_1() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30)
    }
}

fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input));
}

fn process(input: &str) -> usize {
    todo!("Implement process");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 0);
    }
}

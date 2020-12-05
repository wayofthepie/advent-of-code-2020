pub mod part_one {
    pub fn solution(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let (row_details, col_details) = line.split_at(7);
                let row = compute_position(row_details, 'F', 0, 127) as usize;
                let col = compute_position(col_details, 'L', 0, 7) as usize;
                row * 8 + col
            })
            .max()
            .unwrap_or(0)
    }

    fn compute_position(definition: &str, left: char, min: u8, max: u8) -> u8 {
        definition
            .chars()
            .fold((min, max), |(min, max), letter| {
                let mid = min + ((max - min) / 2);
                if letter == left {
                    (min, mid)
                } else {
                    (mid + 1, max)
                }
            })
            .0
    }
}
#[cfg(test)]
mod test {
    use super::part_one;
    use std::fs;

    #[test]
    fn should_read_examples_correctly() {
        let s = fs::read_to_string("resources/day5.txt").unwrap();
        assert_eq!(998, part_one::solution(&s));
    }
}

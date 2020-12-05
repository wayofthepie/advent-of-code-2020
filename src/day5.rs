pub mod part_one {
    pub fn solution(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let (row_details, col_details) = line.split_at(7);
                let row = row_details
                    .chars()
                    .fold((0, 127), |(min, max), letter| {
                        let mid = min + ((max - min) / 2);
                        match letter {
                            'F' => (min, mid),
                            'B' => (mid + 1, max),
                            _ => unreachable!(),
                        }
                    })
                    .0;
                let col = col_details
                    .chars()
                    .fold((0, 7), |(min, max), letter| {
                        let mid = min + ((max - min) / 2);
                        match letter {
                            'L' => (min, mid),
                            'R' => (mid + 1, max),
                            _ => unreachable!(),
                        }
                    })
                    .0;
                row * 8 + col
            })
            .max()
            .unwrap_or(0)
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

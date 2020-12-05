pub mod part_one {
    use super::compute_positions;

    pub fn solution(input: &str) -> usize {
        compute_positions(input).max().unwrap_or(0)
    }
}

pub mod part_two {
    use super::compute_positions;

    pub fn solution(input: &str) -> usize {
        let mut ids = compute_positions(input).collect::<Vec<usize>>();
        ids.sort_unstable();
        ids.windows(2)
            .find(|window| window[0] + 1 < window[1])
            .map(|window| window[0])
            .unwrap_or(0)
            + 1
    }
}

fn compute_positions<'input>(input: &'input str) -> impl Iterator<Item = usize> + 'input {
    input
        .lines()
        .map(compute_row_and_col)
        .map(|(row, col)| row as usize * 8 + col as usize)
}

fn compute_row_and_col(line: &str) -> (u8, u8) {
    let (row_details, col_details) = line.split_at(7);
    (
        compute_position(row_details, 'F', &[64, 32, 16, 8, 4, 2, 1]),
        compute_position(col_details, 'L', &[4, 2, 1]),
    )
}

fn compute_position(definition: &str, left: char, mapping: &[u8]) -> u8 {
    definition
        .chars()
        .zip(mapping)
        .fold(0, |acc, (ch, &n)| if ch == left { acc } else { acc + n })
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};
    use std::fs;

    #[test]
    fn part_one_should_solve() {
        let s = fs::read_to_string("resources/day5.txt").unwrap();
        assert_eq!(998, part_one::solution(&s));
    }

    #[test]
    fn part_two_should_solve() {
        let s = fs::read_to_string("resources/day5.txt").unwrap();
        assert_eq!(676, part_two::solution(&s));
    }
}

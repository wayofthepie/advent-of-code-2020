pub fn solution(terrain: &str) -> usize {
    terrain
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| line.chars().nth((idx * 3) % line.len()))
        .filter(|&c| c == '#')
        .count()
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn should_give_right_answer() {
        let terrain = std::fs::read_to_string("resources/day3.txt").unwrap();
        assert_eq!(218, solution(&terrain))
    }
}

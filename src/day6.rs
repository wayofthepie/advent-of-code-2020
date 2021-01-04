pub mod part_one {
    pub fn solution(input: &str) -> usize {
        input
            .split("\n\n")
            .map(|s| {
                s.chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<Vec<char>>()
            })
            .map(|mut chars| {
                chars.sort_unstable();
                chars.dedup();
                chars.len()
            })
            .sum::<usize>()
    }
}

#[cfg(test)]
mod test {
    use super::part_one;

    #[test]
    pub fn part_one_solution_for_example() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        let result = part_one::solution(input);
        assert_eq!(11, result);
    }

    #[test]
    pub fn part_one_solution() {
        let input = include_str!("../resources/day6.txt");
        let result = part_one::solution(input);
        assert_eq!(6596, result);
    }
}

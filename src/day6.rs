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

pub mod part_two {
    pub fn solution(input: &str) -> usize {
        let groups = input
            .split("\n\n")
            .map(|s| s.split('\n').collect())
            .collect::<Vec<Vec<&str>>>();
        let mut sum = 0;
        for group in groups.iter() {
            let first = group.first().unwrap_or(&"");
            for choice in first.chars() {
                if group.iter().all(|other| other.contains(choice)) {
                    sum += 1;
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    const EXAMPLE: &str = r#"abc

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

    #[test]
    pub fn part_one_solution_for_example() {
        let result = part_one::solution(EXAMPLE);
        assert_eq!(11, result);
    }

    #[test]
    pub fn part_one_solution() {
        let input = include_str!("../resources/day6.txt");
        let result = part_one::solution(input);
        assert_eq!(6596, result);
    }

    #[test]
    pub fn part_two_solution_for_example() {
        let result = part_two::solution(EXAMPLE);
        assert_eq!(6, result);
    }

    #[test]
    pub fn part_two_solution() {
        let input = include_str!("../resources/day6.txt");
        let result = part_two::solution(input);
        assert_eq!(3219, result);
    }
}

use std::cmp::Ordering;

pub fn part_one_solution(s: &str, find: usize) -> usize {
    let mut nums = s
        .split('\n')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    nums.sort_unstable();
    for (i_idx, &i) in nums.iter().enumerate() {
        if let Some(found) = binary_search(&nums[i_idx..], i, find) {
            return i * found;
        }
    }
    0
}

pub fn part_two_solution(s: &str, find: usize) -> usize {
    let mut nums = s
        .split('\n')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    nums.sort_unstable();
    for (i_idx, i) in nums.iter().enumerate() {
        for (j_idx, j) in nums[i_idx..].iter().enumerate() {
            if i + j > find {
                break;
            }
            if let Some(found) = binary_search(&nums[j_idx..], i + j, find) {
                return i * j * found;
            }
        }
    }
    0
}

fn binary_search(mut nums: &[usize], sum: usize, find: usize) -> Option<usize> {
    let mut mid = usize::MAX;
    while mid != 0 {
        mid = nums.len() / 2;
        match (sum + nums[mid]).cmp(&find) {
            Ordering::Less => nums = &nums[mid..],
            Ordering::Greater => nums = &nums[..mid],
            Ordering::Equal => return Some(nums[mid]),
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::{part_one_solution, part_two_solution};
    use std::fs;

    #[test]
    fn validate_part_one() {
        let s = fs::read_to_string("resources/day1.txt").expect("");
        assert_eq!(974304, part_one_solution(&s, 2020))
    }

    #[test]
    fn validate_part_two() {
        let s = fs::read_to_string("resources/day1.txt").expect("");
        assert_eq!(236430480, part_two_solution(&s, 2020));
    }

    #[test]
    fn validate_part_two_worst() {
        let s = fs::read_to_string("resources/day1_worst.txt").expect("");
        assert_eq!(4000000, part_two_solution(&s, 2000003));
    }
}

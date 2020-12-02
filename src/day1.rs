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
pub fn part_two_solution_loops(s: &str, find: usize) -> usize {
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
            for k in nums.iter() {
                let sum = i+j+k;
                if sum > find {
                    break;
                }
                if sum == find {
                    return i*j*k;
                }
            }
        }
    }
    0
}

pub fn part_two_solution_single_binsearch(s: &str, find: usize) -> usize {
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

pub fn part_two_solution_2d_binsearch(s: &str, find: usize) -> usize {
    let mut nums = s
        .split('\n')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    nums.sort_unstable();
    for (i_idx, &i) in nums.iter().enumerate() {
        if let Some(found) = outer_binary_search(&nums[i_idx..], &nums[i_idx + 1..], i, find) {
            return found;
        }
    }
    0
}

fn outer_binary_search(
    mut outer: &[usize],
    inner: &[usize],
    sum: usize,
    find: usize,
) -> Option<usize> {
    let mut mid = usize::MAX;
    while mid != 0 {
        mid = outer.len() / 2;
        match inner_binary_search(&inner, sum + outer[mid], find) {
            Some((Ordering::Less, _)) => outer = &outer[mid..],
            Some((Ordering::Greater, _)) | None => outer = &outer[..mid],
            Some((Ordering::Equal, found)) => return Some(sum * outer[mid] * found),
        }
    }
    None
}

fn inner_binary_search(mut nums: &[usize], sum: usize, find: usize) -> Option<(Ordering, usize)> {
    let mut mid = usize::MAX;
    let mut last_ord = None;
    while mid != 0 {
        if nums.is_empty() {
            return None;
        }
        mid = nums.len() / 2;
        match (sum + nums[mid]).cmp(&find) {
            ord @ Ordering::Less => {
                last_ord = Some(ord);
                nums = &nums[mid..];
            }
            ord @ Ordering::Greater => {
                last_ord = Some(ord);
                nums = &nums[..mid];
            }
            ord @ Ordering::Equal => return Some((ord, nums[mid])),
        }
    }
    if let Some(ord) = last_ord {
        Some((ord, 0))
    } else {
        None
    }
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
    use super::{
        part_one_solution, part_two_solution_2d_binsearch, part_two_solution_single_binsearch,
    };
    use std::fs;

    #[test]
    fn validate_part_one() {
        let s = fs::read_to_string("resources/day1.txt").expect("");
        assert_eq!(974304, part_one_solution(&s, 2020))
    }

    #[test]
    fn validate_part_two() {
        let s = fs::read_to_string("resources/day1.txt").expect("");
        assert_eq!(236430480, part_two_solution_2d_binsearch(&s, 2020));
        assert_eq!(236430480, part_two_solution_single_binsearch(&s, 2020));
    }

    #[test]
    fn validate_part_two_worst() {
        let s = fs::read_to_string("resources/day1_worst2.txt").expect("");
        assert_eq!(40000000, part_two_solution_2d_binsearch(&s, 20000003));
        assert_eq!(40000000, part_two_solution_single_binsearch(&s, 20000003));
    }
}

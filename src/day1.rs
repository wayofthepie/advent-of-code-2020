use std::{error, fs};

pub fn part_one_solution() -> Result<usize, Box<dyn error::Error>> {
    let s = fs::read_to_string("resources/day1.txt")?;
    let nums = s
        .split('\n')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    for i in nums.iter() {
        for j in nums.iter() {
            if i + j == 2020 {
                return Ok(i * j);
            }
        }
    }
    Ok(0)
}

pub fn part_two_solution() -> Result<usize, Box<dyn error::Error>> {
    let s = fs::read_to_string("resources/day1.txt")?;
    let nums = s
        .split('\n')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    for i in nums.iter() {
        for j in nums.iter() {
            for k in nums.iter() {
                if i + j + k == 2020 {
                    return Ok(i * j * k);
                }
            }
        }
    }
    Ok(0)
}

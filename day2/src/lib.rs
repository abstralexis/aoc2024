//! Day 2 challenges. The north pole shall become the new chernobyl if I fail.

pub fn get_input() -> Vec<String> {
    let input = include_str!("input.txt");
    let input = input.lines().map(|x| x.to_string()).collect();
    input
}

pub fn is_increasing(input: String) -> bool {
    let mut sort_vec = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    sort_vec.sort();
    sort_vec == input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

pub fn is_decreasing(input: String) -> bool {
    let mut sort_vec = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    sort_vec.sort();
    sort_vec.reverse();
    sort_vec == input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

pub fn are_diffs_safe(input: String) -> bool {
    let nums: Vec<i32> = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    for i in 1..(nums.len())  {
        let diff = (nums[i] - nums[i-1]).abs();
        if diff > 3 || diff < 1 {
            return false;
        }
    }
    true
} 

pub fn part1() {
    let num_safe = get_input().iter().filter(|s| 
        are_diffs_safe(s.to_string()) && 
        (is_increasing(s.to_string()) || is_decreasing(s.to_string()))
    ).count();
    println!("{}", num_safe);
}

pub fn part2() {
    let num_safe = get_input().iter().filter(|s| check_dampened(s.to_string())).count();
    println!("{}", num_safe);
}

pub fn check_dampened(input: String) -> bool {
    let nums: Vec<i32> = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    let mut permutations = vec![];
    permutations.push(nums.clone());
    for i in 0..nums.len() {
        let mut perm = nums.clone();
        perm.remove(i);
        permutations.push(perm);
    }
    for perm in permutations {
        if are_diffs_safe(perm.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")) &&
            (is_increasing(perm.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")) ||
             is_decreasing(perm.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "))) {
            return true;
        }
    }
    false
}
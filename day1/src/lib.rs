//! Day 1 Stuff!

use std::io::Read;
use rayon::prelude::*;
use std::collections::HashMap;
use itertools::Itertools;

/// Parse the input into a vector of vectors of i32s.
pub fn parse_pair(input: &str) -> Vec<(i32, i32)> {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }
    left.sort();
    right.sort();
    left.into_iter().zip(right.into_iter()).collect()
}

pub fn parse_seperate(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }
    (left, right)
}

/// Sum the differences between the pairs of numbers in the input.
/// Assumes the input is a vec of pairs of i32s as established by the parse function.
pub fn sum_differences(input: Vec<(i32, i32)>) -> i32 {
    input.iter().map(|pair| (pair.0 - pair.1).abs()).sum()
}

pub fn calculate_similarity(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let (left, right) = input;
    let right_counts = right.iter().counts();
    let mut similarity = 0;
    for num in left {
        if right_counts.contains_key(&num) {
            similarity += num * right_counts[&num] as i32;
        }
    }
    similarity
}

pub fn part1() {
    let input: String = std::fs::File::open("day1/src/input.txt")
        .unwrap()
        .bytes()
        .map(|b| b.unwrap() as char)
        .collect();
    let parsed = parse_pair(&input);
    println!("{:?}", sum_differences(parsed));
}

pub fn part2() {
    let input: String = std::fs::File::open("day1/src/input.txt")
        .unwrap()
        .bytes()
        .map(|b| b.unwrap() as char)
        .collect();
    let parsed = parse_seperate(&input);
    println!("{:?}", calculate_similarity(parsed));
}

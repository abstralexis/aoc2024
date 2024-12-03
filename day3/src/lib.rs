//! Day 3 - Corrupted Memory
use std::usize;

use regex::Regex;

pub fn part1() {
    let input = include_str!("input.txt");
    println!("{}", clean_up(input).iter().fold(0, |acc, x| acc + get_mul_and_exec(x.to_string())));
}

/// Get the input and *just* get the vector of all the multiplication calls.
pub fn clean_up(s: &str) -> Vec<String>{
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let cleaned = re.find_iter(&s)
        .map(|x| x.as_str().to_string())
        .collect();
    return cleaned;
}

pub fn get_mul_and_exec(mul: String) -> i32 {
    let re = Regex::new(r"\d{1,3}").unwrap();
    let mut nums = re.find_iter(&mul)
        .map(|x| x.as_str().parse::<i32>().unwrap())
        .collect::<Vec<i32>>(); // we can safely assume just 2 elements as its already a match
    nums.pop().unwrap() * nums.pop().unwrap()
}

pub fn part2() {
    // strip those between `don't()` and `do()` then reapply get_mul_and_exec
    // // let input = include_str!("input.txt");

    // regex wizardry. i must learn these new lookbehind and "lazy" spells.
    // but rust doesnt support it! shit.
    // // let re = Regex::new(r"(?s)(?<=don't\(\)).*?(?=do\(\))").unwrap();
    // // let mut do_dont = re.replace_all(input, "").to_string();

    // we shall do the regexing online
    let input = include_str!("result.txt");

    println!("{}", clean_up(&input).iter().fold(0, |acc, x| acc + get_mul_and_exec(x.to_string())))
}


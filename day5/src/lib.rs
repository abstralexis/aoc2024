//! Day 5 of Advent of Code!
use itertools::Itertools;

#[derive(Debug)]
pub struct Rule {
    pub first: i32,
    pub second: i32,
}
impl Rule {
    pub fn new(first: i32, second: i32) -> Rule {
        Rule { first, second }
    }

    pub fn fetch_rules() -> Vec<Rule> {
        let mut rules = vec![];
        let input = include_str!("rules.txt");
        for line in input.lines() {
            let mut split = line.split("|");
            let first = split.next().unwrap();
            let second = split.next().unwrap();
            rules.push(Rule::new(first.parse::<i32>().unwrap(), second.parse::<i32>().unwrap()));
        }
        rules
    }

    pub fn does_break_rule(&self, update: &Vec<i32>) -> bool {
        for i in 0..update.len() {
            if update[i] == self.first {
                if update[..i].contains(&self.second) { return true };
            }
        }
        false
    }
}

pub fn fetch_updates() -> Vec<Vec<i32>> {
    let input = include_str!("updates.txt");
    input.lines().map(|line| line.split(",").map(|x| x.parse::<i32>().unwrap()).collect()).collect()
}

pub fn part1() {
    let rules = Rule::fetch_rules();
    let updates = fetch_updates();

    let filtered = updates.iter().filter(|update| {
        for rule in &rules {
            if rule.does_break_rule(update) { return false }
        }
        true
    }).collect::<Vec<_>>();

    let sum = filtered.iter().map(|update| update[update.len()/2]).sum::<i32>();
    println!("Part 1: {}", sum);
}

pub fn part2() {
    let rules = Rule::fetch_rules();
    let updates = fetch_updates();

    // now we have the evil updates!
    let filtered = updates.iter().filter(|update| {
        for rule in &rules {
            if rule.does_break_rule(update) { return true }
        }
        false
    }).collect::<Vec<_>>();

    let mut sorted: Vec<Vec<i32>> = vec![];
    for update in filtered {
        let mut up = update.clone();

        let mut is_sorted = false;
        while !is_sorted {
            is_sorted = true;
            for rule in &rules {
                if rule.does_break_rule(&up) {
                    let first_idx = up.iter().position(|&x| x == rule.first).unwrap();
                    let second_idx = up.iter().position(|&x| x == rule.second).unwrap();
                    let first_val = up[first_idx];
                    let second_val = up[second_idx];
                    up[first_idx] = second_val;
                    up[second_idx] = first_val;
                    is_sorted = false;
                }
            }
        }

        sorted.push(up);
    }

    let sum = sorted.iter().map(|update| update[update.len()/2]).sum::<i32>();
    println!("Part 2: {}", sum);
}

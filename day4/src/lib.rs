//! Day 4 of AoC!

fn get_input() -> Vec<String> {
    let input = include_str!("input.txt");
    input.lines().map(|x| x.to_string()).collect()
}

pub fn part1() {
    let input = get_input();
    let mut count = 0;

    let mut horizontals = vec![];
    let mut verticals = vec![];
    let mut diagonals = vec![];

    // Extract horizontals
    for i in 0..input.len() {
        for j in 0..input[i].len() - 3 {
            horizontals.push(input[i].chars().skip(j).take(4).collect::<Vec<char>>());
        }
    }

    // Extract verticals
    for i in 0..input.len() - 3 {
        for j in 0..input[i].len() {
            let mut vertical = vec![];
            for k in 0..4 {
                vertical.push(input[i + k].chars().nth(j).unwrap());
            }
            verticals.push(vertical);
        }
    }

    // Extract diagonals (top-left to bottom-right)
    for i in 0..input.len() - 3 {
        for j in 0..input[i].len() - 3 {
            let mut diagonal = vec![];
            for k in 0..4 {
                diagonal.push(input[i + k].chars().nth(j + k).unwrap());
            }
            diagonals.push(diagonal);
        }
    }

    // Extract diagonals (top-right to bottom-left)
    for i in 0..input.len() - 3 {
        for j in 3..input[i].len() {
            let mut diagonal = vec![];
            for k in 0..4 {
                diagonal.push(input[i + k].chars().nth(j - k).unwrap());
            }
            diagonals.push(diagonal);
        }
    }

    // Search for patterns
    let patterns = vec![
        vec!['X', 'M', 'A', 'S'],
        vec!['S', 'A', 'M', 'X'],
    ];

    for pattern in patterns {
        for horizontal in &horizontals {
            if horizontal == &pattern {
                count += 1;
            }
        }
        for vertical in &verticals {
            if vertical == &pattern {
                count += 1;
            }
        }
        for diagonal in &diagonals {
            if diagonal == &pattern {
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);
}

pub fn part2() {
    let input = get_input();

    let mut count = 0;

    // get a sliding window 3x3 
    for i in 0..input[0].len()-2 {
        for j in 0..input.len()-2 {
            let window = vec![
                input[j].chars().skip(i).take(3).collect::<Vec<char>>(),
                input[j+1].chars().skip(i).take(3).collect::<Vec<char>>(),
                input[j+2].chars().skip(i).take(3).collect::<Vec<char>>(),
            ];

            // check if *both* diagonals are MAS forwards or backwards
            let is_main_diag_mas = window[0][0] == 'M' && window[1][1] == 'A' && window[2][2] == 'S';
            let is_main_diag_sam = window[0][0] == 'S' && window[1][1] == 'A' && window[2][2] == 'M';
            let is_main_diag = is_main_diag_mas || is_main_diag_sam;

            let is_anti_diag_mas = window[0][2] == 'M' && window[1][1] == 'A' && window[2][0] == 'S';
            let is_anti_diag_sam = window[0][2] == 'S' && window[1][1] == 'A' && window[2][0] == 'M';
            let is_anti_diag = is_anti_diag_mas || is_anti_diag_sam;

            if is_main_diag && is_anti_diag {
                count += 1;
            }
        }
    }

    println!("Part 2: {}", count);
}

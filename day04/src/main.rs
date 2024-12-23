use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", task_01(&input));
    println!("{}", task_02(&input));
}

fn task_01(input: &str) -> i32 {
    let mut count = 0;
    let char_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for (x, line) in char_map.iter().enumerate() {
        for (y, char) in line.iter().enumerate() {
            if *char == 'X' {
                if y >= 3 {
                    // Left
                    if line[y - 1] == 'M' && line[y - 2] == 'A' && line[y - 3] == 'S' {
                        count += 1
                    }

                    if x >= 3 {
                        // Top left
                        if char_map[x - 1][y - 1] == 'M'
                            && char_map[x - 2][y - 2] == 'A'
                            && char_map[x - 3][y - 3] == 'S'
                        {
                            count += 1;
                        }
                    }

                    if x + 3 < char_map.len() {
                        // Bottom left
                        if char_map[x + 1][y - 1] == 'M'
                            && char_map[x + 2][y - 2] == 'A'
                            && char_map[x + 3][y - 3] == 'S'
                        {
                            count += 1
                        }
                    }
                }
                if y + 3 < line.len() {
                    // Right
                    if line[y + 1] == 'M' && line[y + 2] == 'A' && line[y + 3] == 'S' {
                        count += 1
                    }

                    if x >= 3 {
                        // Top Right
                        if char_map[x - 1][y + 1] == 'M'
                            && char_map[x - 2][y + 2] == 'A'
                            && char_map[x - 3][y + 3] == 'S'
                        {
                            count += 1;
                        }
                    }

                    if x + 3 < char_map.len() {
                        // Bottom right
                        if char_map[x + 1][y + 1] == 'M'
                            && char_map[x + 2][y + 2] == 'A'
                            && char_map[x + 3][y + 3] == 'S'
                        {
                            count += 1
                        }
                    }
                }
                if x >= 3 {
                    // Up
                    if char_map[x - 1][y] == 'M'
                        && char_map[x - 2][y] == 'A'
                        && char_map[x - 3][y] == 'S'
                    {
                        count += 1
                    }
                }
                if x + 3 < char_map.len() {
                    // Down
                    if char_map[x + 1][y] == 'M'
                        && char_map[x + 2][y] == 'A'
                        && char_map[x + 3][y] == 'S'
                    {
                        count += 1
                    }
                }
            }
        }
    }

    return count;
}

fn task_02(input: &str) -> i32 {
    let mut count = 0;
    let char_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for x in 1..char_map.len() - 1 {
        for y in 1..char_map[x].len() - 1 {
            if char_map[x][y] != 'A' {
                continue;
            };

            let surrounding_vec: Vec<char> = vec![
                char_map[x - 1][y - 1],
                char_map[x - 1][y + 1],
                char_map[x + 1][y - 1],
                char_map[x + 1][y + 1],
            ];

            let allowed = [
                vec!['M', 'S', 'M', 'S'],
                vec!['S', 'M', 'S', 'M'],
                vec!['S', 'S', 'M', 'M'],
                vec!['M', 'M', 'S', 'S'],
            ];

            if allowed.contains(&surrounding_vec) {
                count += 1;
            }
        }
    }

    return count;
}

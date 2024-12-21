use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

fn main() {
    let input_file = File::open("input.txt").expect("Failed to find input.txt");
    let buf_reader = BufReader::new(input_file);
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();

    for line in buf_reader.lines() {
        let line = line.expect("Failed to get line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        list_a.push(
            parts[0]
                .parse::<i32>()
                .expect("Failed to parse int, is the input correct?"),
        );
        list_b.push(
            parts[1]
                .parse::<i32>()
                .expect("Failed to parse int, is the input correct?"),
        );
    }

    part_one(list_a.clone(), list_b.clone());
    part_two(list_a, list_b);
}

fn part_one(mut left: Vec<i32>, mut right: Vec<i32>) {
    left.sort();
    right.sort();

    let mut total_distance = 0;

    for (left, right) in left.iter().zip(right.iter()) {
        total_distance += (left - right).abs();
    }

    println!("Total difference: {}", total_distance);
}

fn part_two(left: Vec<i32>, right: Vec<i32>) {
    let mut frequencies: HashMap<i32, i32> = HashMap::new();
    let mut sim_score = 0;

    for num in right {
        *frequencies.entry(num).or_default() += 1;
    }

    for num in left {
        let freq = *frequencies.entry(num).or_default();
        sim_score += num * freq;
    }

    println!("Sim score: {}", sim_score);
}

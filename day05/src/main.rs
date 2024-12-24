use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fs::{self},
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", task_01(&input));
}

fn task_01(input: &str) -> i32 {
    let mut result = 0;
    // Given a i32 map to i32s wich have to be after it
    let mut rulemap: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut update_cores: Vec<i32> = Vec::new();

    let (rules, updates) = input
        .split_once("\n\n")
        .expect("Failed to split at emptyline");

    for rule in rules.lines() {
        let (num, num_after) = rule.split_once('|').expect("Failed to parse rule, no |?");
        let num = num.parse::<i32>().expect("Failed to parse left num");
        let num_after = num_after.parse::<i32>().expect("Failed to parse right num");

        match rulemap.entry(num) {
            Entry::Vacant(entry) => {
                let mut set = HashSet::new();
                set.insert(num_after);
                entry.insert(set);
            }
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(num_after);
            }
        }
    }

    for update in updates.lines() {
        let mut seen_numbers: HashSet<i32> = HashSet::new();
        let raw_numbers = update.split(',');
        let numbers: Vec<i32> = raw_numbers.map(|num| num.parse::<i32>().unwrap()).collect();
        let mut valid = true;

        for number in &numbers {
            match rulemap.get(&number) {
                Some(nums_after) => {
                    for num in &seen_numbers {
                        if nums_after.contains(&num) {
                            valid = false;
                            break;
                        }
                    }
                }
                None => {}
            };
            seen_numbers.insert(*number);
        }
        if valid {
            update_cores.push(numbers[numbers.len() / 2]);
            result += 1;
        }
    }

    update_cores.iter().fold(0, |acc, val| acc + val)
}

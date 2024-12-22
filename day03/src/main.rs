use std::{fs::File, io::BufReader, io::Read};

use regex::Regex;

// mul\([0-9]+,[0-9]+\)
fn main() {
    let input_file = File::open("input.txt").expect("Failed to find input.txt");
    let mut buf_reader = BufReader::new(input_file);
    let mut input_content = String::new();

    buf_reader
        .read_to_string(&mut input_content)
        .expect("Failed to read input to string");

    let res_1 = task_01(input_content.clone());
    let res_2 = task_02(input_content);

    println!("Res 1: {res_1}, res 2: {res_2}");
}

fn task_01(input_content: String) -> i32 {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").expect("Failed to compile regex");
    let regex_matches: Vec<(i32, i32)> = re
        .find_iter(&input_content)
        .map(|rmatch| {
            let raw_str = rmatch.as_str();
            let cleaned_string = raw_str.replace("mul(", "").replace(")", "");
            let parts: Vec<i32> = cleaned_string
                .split(',')
                .map(|x| x.parse::<i32>().expect("Failed to parse int"))
                .collect();

            (parts[0], parts[1])
        })
        .collect();

    let result = regex_matches
        .into_iter()
        .fold(0, |acc, x| acc + (x.0 * x.1));

    return result;
}

fn task_02(input_content: String) -> i32 {
    let re_do = Regex::new(r"do\(\)").expect("Failed to compile regex 1");
    let re_dont = Regex::new(r"don't\(\)").expect("Failed to compile regex 2");

    let mut current_index = 0;
    let mut result = 0;
    let mut dont_match = re_dont.find(&input_content);

    if dont_match.is_none() {
        return task_01(input_content);
    }

    while dont_match.is_some() {
        let next_do = re_do.find_at(&input_content, dont_match.unwrap().end());

        match next_do {
            Some(next_do) => {
                let substring = &input_content[current_index..dont_match.unwrap().start()];
                result += task_01(substring.to_string());

                current_index = next_do.end();

                dont_match = re_dont.find_at(&input_content, current_index);
                if dont_match.is_none() {
                    let rest_of_string = &input_content[current_index..];
                    result += task_01(rest_of_string.to_string());
                }
            }
            None => return result,
        }
    }
    return result;
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq)]
enum ReportType {
    INCREASING,
    DECREASING,
    UNKOWN,
}

fn main() {
    let input_file = File::open("input.txt").expect("Failed to find input.txt");
    let buf_reader = BufReader::new(input_file);
    let mut safe_reports = 0;
    let mut safe_reports_2 = 0;

    for line in buf_reader.lines() {
        let report: Vec<i32> = line
            .expect("Failed to get line from file")
            .split_whitespace()
            .map(|x| {
                x.parse::<i32>()
                    .expect("Failed to parse int, is the input file correct?")
            })
            .collect();

        safe_reports += check_report(&report).0;
        safe_reports_2 += check_report_2(&report);
    }

    println!("Safe reports: {}", safe_reports);
    println!("Safe reports v2: {}", safe_reports_2);
}

fn check_report(report: &Vec<i32>) -> (i32, usize) {
    let mut report_type = ReportType::UNKOWN;
    for (i, pair) in report.windows(2).enumerate() {
        let diff = (pair[0] - pair[1]).abs();
        if diff < 1 || diff > 3 {
            return (0, i);
        }

        if pair[0] > pair[1] {
            match report_type {
                ReportType::UNKOWN => report_type = ReportType::DECREASING,
                ReportType::INCREASING => return (0, i),
                ReportType::DECREASING => continue,
            }
        }

        if pair[1] > pair[0] {
            match report_type {
                ReportType::UNKOWN => report_type = ReportType::INCREASING,
                ReportType::INCREASING => continue,
                ReportType::DECREASING => return (0, i),
            }
        }
    }
    return (1, 0);
}

fn check_report_2(report: &Vec<i32>) -> i32 {
    let res = check_report(&report);
    if res.0 == 1 {
        return 1;
    } else {
        for i in 0..report.len() {
            let mut rep = report.clone();
            rep.remove(i);
            if check_report(&rep).0 == 1 {
                return 1;
            }
        }
        return 0;
    }
}

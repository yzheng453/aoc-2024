use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let lines = read_lines("input.txt");
    let n_safe_reports: i64 = lines.iter().map(|line| {
        let raw_report: Vec<i64> = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
        for i_bad_level in 0..raw_report.len() {
            let report = [&raw_report[..i_bad_level], &raw_report[(i_bad_level+1)..]].concat();
            if test_report(report) == 1 {
                return 1;
            }
        }
        0
    }).sum();
    println!("{}", n_safe_reports);

}

fn test_report(report: Vec<i64>) -> i64 {
    let direction = if report[0] < report[1] {
        -1
    } else {
        if report[0] > report[1] {
            1
        } else {
            return 0;
        }
    };
    for i in 1..report.len() {
        let delta = (report[i-1] - report[i]) * direction;            
        if (delta < 1) || (delta > 3) {
            return 0;
        }
    }
    1
}


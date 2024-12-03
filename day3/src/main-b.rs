use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let input_s = read_to_string("input.txt").unwrap();
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut sum: i64 = 0;
    let mut enabled = true;
    for cm in re.captures_iter(&input_s) {
        let s = cm.get(0).unwrap().as_str();
        if s.starts_with(r"do(") {
            enabled = true;
        } else if s.starts_with(r"don't(") {
            enabled = false;
        } else if enabled {
            let xs = cm.get(1).unwrap().as_str();
            let ys = cm.get(2).unwrap().as_str();
            
            let x: i64 = xs.parse().unwrap();
            let y: i64 = ys.parse().unwrap();
            sum += x * y;
        }
    }
    println!("{}", sum);

}


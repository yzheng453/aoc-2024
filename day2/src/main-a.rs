use std::fs::read_to_string;
use std::iter::zip;
//use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let lines = read_lines("input.txt");
    let sum: i64 = lines.iter().map(|line| {
        let a: Vec<i64> = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
        let d = if a[0] < a[1] {
            -1
        } else {
            if a[0] > a[1] {
                1
            } else {
                return 0;
            }
        };
        for i in 1..a.len() {
            let m = (a[i-1] - a[i]) * d;            
            if (m < 1) || (m > 3) {
                return 0;
            }
        }
        return 1;
    }).sum();
    println!("{}", sum);

}


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
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let sum: i64 = re.captures_iter(&input_s).map(|captures| {
        let (_, [xs, ys]) = captures.extract();
        let x: i64 = xs.parse().unwrap();
        let y: i64 = ys.parse().unwrap();
        x * y
    }).sum();
    println!("{}", sum);

}


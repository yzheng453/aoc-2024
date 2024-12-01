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
    let lines = read_lines("input");
    let (numbers_l, numbers_r) : (Vec<_>, Vec<_>) = lines.iter().map(|line| {
        let mut s = line.split("   ");
        let n1 = s.next().unwrap().parse::<i64>().unwrap();
        let n2 = s.next().unwrap().parse::<i64>().unwrap();
        (n1, n2)
    }).unzip();

    let mut cnt = vec![0; 1000_000];

    for n in numbers_r {
        cnt[n as usize] += 1;
    }

    let total_dist: i64 = numbers_l.iter().map(|n| n * cnt[*n as usize]).sum();

    println!("{}", total_dist);

}


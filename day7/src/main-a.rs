use std::fs::read_to_string;

fn main() {
    
    let s: i64 = read_lines("input.txt").into_iter().map(|line| {
        let mut i = line.split(": ");
        let test_value: i64 = i.next().unwrap().parse().unwrap();
        let numbers: Vec<i64> = i.next().unwrap().split(' ').map(|s| s.parse().unwrap()).collect();
        
        if can_evaluate(test_value, &numbers) {
            test_value
        } else {
            0
        }
    }).sum();
    
    println!("{}", s);
}

fn can_evaluate(test_value: i64, numbers: &[i64]) -> bool {
    sch(test_value, &numbers[1..], numbers[0])
}

fn sch(test_value: i64, numbers: &[i64], v: i64) -> bool {
    if numbers.len() == 0 {
        return test_value == v;
    }
    sch(test_value, &numbers[1..], v + numbers[0]) || sch(test_value, &numbers[1..], v * numbers[0])
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
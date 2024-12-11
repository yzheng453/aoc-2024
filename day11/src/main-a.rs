use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

fn main() {
    
    let input: Vec<i64> = read_lines("input.txt").iter().next().map(|s| {
        s.split(' ').map(|n| n.parse::<i64>().unwrap()).collect()
    }).unwrap();
    
    let mut sum = 0;
    let mut memory = BTreeMap::new();
    for n in input {
        sum += sch(&mut memory, n as u64, 25);
    }
    
    println!("{}", sum);
}


fn sch(memory: &mut BTreeMap<(u64, i64), i64>, n: u64, blinks: i64) -> i64 {
    if blinks == 0 {
        return 1;
    }
    if let Some(r) = memory.get(&(n, blinks)) {
        return *r;
    }
    let r = if n == 0 {
        sch(memory, 1, blinks - 1)
    } else if let Some((l, r)) = split(n) {
        let v1 = sch(memory, l, blinks - 1);
        let v2 = sch(memory, r, blinks - 1);
        v1 + v2
    } else {
        sch(memory, n * 2024, blinks - 1)
    };
    memory.insert((n, blinks), r);
    return r;
}

fn split(input: u64) -> Option<(u64, u64)> {
    let mut digits = 0;
    let mut n = input;
    while n > 0 {
        n = n / 10;
        digits += 1;
    }
    if digits % 2 == 0 {
        let mut l = 0;
        let mut r = 0;
        n = input;
        let mut d = 1;

        for i in 0.. digits/2 {
            r = r + (n % 10) * d;
            d *= 10;
            n = n / 10;
        }

        d = 1;
        for i in 0.. digits/2 {
            l = l + (n % 10) * d;
            d *= 10;
            n = n / 10;
        }
        
        return Some((l, r));
    }
    return None;
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
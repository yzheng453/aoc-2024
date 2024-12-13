use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

use regex::Regex;

fn main() {
    let button_re = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut it = read_lines("input.txt").into_iter();
    let mut total_cost = 0;
    while it.next().is_some() {
        let astring = it.next().unwrap();
        let bstring = it.next().unwrap();
        let (_, [xa, ya]) = button_re.captures(&astring).unwrap().extract();
        let (xa, ya): (i64, i64) = (xa.parse().unwrap(), ya.parse().unwrap());

        let (_, [xb, yb]) = button_re.captures(&bstring).unwrap().extract();
        let (xb, yb): (i64, i64) = (xb.parse().unwrap(), yb.parse().unwrap());

        let prize = it.next().unwrap();
        let (_, [x, y]) = prize_re.captures(&prize).unwrap().extract();
        let (x, y): (i64, i64) = (x.parse().unwrap(), y.parse().unwrap());
        let (x, y) = (x + 10000000000000, y + 10000000000000);
        
        let det = (xb*ya - yb*xa);
        if det != 0 {
            let b = (x*ya - y*xa) / det;
            let a = (-x*yb + y*xb) / det;
            if validate(a, b, xa, xb, ya, yb, x, y) {
                let cost = 3 * a + b;
                println!("{} {} {}", cost, a, b);
                total_cost += cost;
            }
        } else {
            let (p, q, g) = gcd(xa, xb);
            if x % g != 0 || y % g != 0 {
                continue;
            }
            
            let k = if 3 * xb - xa > 0 {
                - g * p / xb
            } else {
                g * q / xa
            };
            let a = p + xb * k / g;
            let b = q + xa * k / g;
            total_cost += 3 * a + b;
        }
    }
    
    println!("{}", total_cost);
}

fn validate(a: i64, b: i64, xa: i64, xb: i64, ya: i64, yb: i64, x: i64, y: i64) -> bool {
    if (a < 0) || (b < 0) {
        return false;
    }

    if a * xa + b * xb != x {
        return false;
    }
    
    if a * ya + b * yb != y {
        return false;
    }
    
    return true;
}

fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let c = a % b;
    if c == 0 {
        return (0, 1, b);
    }
    let d = a / b;
    let (x, y, g) = gcd(b, c);
    (y, x - d * y, g)
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
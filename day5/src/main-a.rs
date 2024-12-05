use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

fn main() {
    
    let mut rules: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut mode = 0;
    let mut sum: i64 = 0;
    for line in read_lines("input.txt") {
        if (line.len() < 2) {
            mode = 1;
            continue;
        } else if (mode == 0) {
            let mut it = line.split('|');
            let l: i64 = it.next().unwrap().parse().unwrap();
            let r: i64 = it.next().unwrap().parse().unwrap();
            rules.insert((l, r));
        } else {
            let a: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            let correct = (0..a.len()).flat_map(|i| ((i+1)..a.len()).map(move |j| (i, j)))
                .all(|(i,j)| !rules.contains(&(a[j], a[i])));
            if correct {
                sum += a[a.len() / 2];
            }
        }
    }

    println!("{}", sum);

}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
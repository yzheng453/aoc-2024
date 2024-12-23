use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string};

use regex::Regex;

fn main() {
    let mut edges: BTreeMap<(u8, u8), BTreeSet<(u8, u8)>> = BTreeMap::new();
    for s in read_lines("input.txt") {
        let mut i = s.split('-');
        let l = i.next().unwrap().as_bytes();
        let l = (l[0], l[1]);
        let r = i.next().unwrap().as_bytes();
        let r = (r[0], r[1]);
        
        edges.entry(l)
            .and_modify(|v| {v.insert(r);})
            .or_insert_with(|| {
                let mut s = BTreeSet::new();
                s.insert(r);
                s
            });

        edges.entry(r)
            .and_modify(|v| {v.insert(l);})
            .or_insert_with(|| {
                let mut s = BTreeSet::new();
                s.insert(l);
                s
            });
    }
    
    let mut count = 0;
    
    for (&l, rs) in edges.iter() {
        for &r in rs.iter() {
            let ls = &edges[&l];
            let rs = &edges[&r];
            for &k in ls.intersection(rs) {
                if l.0 == b't' || r.0 == b't' || k.0 == b't' {
                    count += 1;
                }                
            }
        }
    }
    
    println!("{}", count / 6);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

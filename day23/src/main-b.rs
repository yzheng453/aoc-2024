use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string, iter, ops::Div};

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
    
    let mut largest = BTreeSet::new();
    for (l, rs) in edges.iter() {
        for r in rs.iter() {
            if *l > *r {
                continue;
            }
            let mut current = BTreeSet::new();
            current.insert(*l);
            current.insert(*r);
            let result = sch(&edges, &mut current, edges[l].intersection(&edges[r]).copied().collect());
            if result.len() > largest.len() {
                largest = result;
            }
        }
    }
    
    for i in largest {
        print!("{}{},", i.0 as char, i.1 as char);
    }
}

fn sch(edges: &BTreeMap<(u8, u8), BTreeSet<(u8, u8)>>, current: &mut BTreeSet<(u8, u8)>, candidates: BTreeSet<(u8, u8)>) -> BTreeSet<(u8, u8)> {
    let mut largest = BTreeSet::new();
    for candidate in candidates.iter() {
        if *candidate < *current.last().unwrap() {
            continue;
        }
        let s = &edges[candidate];
        let mut flag = true;
        for c in current.iter() {
            if !s.contains(c) {
                flag = false;
                break;
            }
        }
        if flag {
            current.insert(*candidate);
            let r = sch(edges, current, candidates.intersection(s).copied().collect());
            if r.len() > largest.len() {
                largest = r;
            }
            current.remove(candidate);
        }
    }
    if largest.len() < current.len() {
        return current.clone();
    }
    return largest;
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
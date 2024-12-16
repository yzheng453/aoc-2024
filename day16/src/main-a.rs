use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};
use std::ops::Bound::Included;

static DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {

    let map: Vec<Vec<u8>> = read_lines("input.txt").into_iter().map(|line| line.into_bytes()).collect();
    let (x, y, _) = map.iter().enumerate().flat_map(|(x, row)| row.iter().enumerate().filter(|b| *b.1 == b'S').map(move |(y, c)| (x as i64, y as i64, c))).next().unwrap();
    
    let mut s = BTreeMap::new();
    s.insert((x, y, 1), 0);
    let mut queue = Vec::new();
    queue.push((x, y, 1));
    while let Some((x, y, d)) = queue.pop() {
        let current_score = *s.get(&(x, y, d)).unwrap();
        update(&mut s, &mut queue, x, y, (d-1).rem_euclid(4), current_score + 1000);
        update(&mut s, &mut queue, x, y, (d+1).rem_euclid(4), current_score + 1000);
        let (dx, dy) = DIRECTIONS[d as usize];
        let (nx, ny) = (x + dx, y + dy);
        if get_from_map(nx, ny, &map).is_some_and(|u| u == b'.' || u == b'E') {
            update(&mut s, &mut queue, nx, ny, d, current_score + 1);
        }
    }

    let (ex, ey, _) = map.iter().enumerate().flat_map(|(x, row)| row.iter().enumerate().filter(|b| *b.1 == b'E').map(move |(y, c)| (x as i64, y as i64, c))).next().unwrap();
    let m = s.range((Included(&(ex, ey, 0)), Included(&(ex, ey, 3)))).map(|e| *e.1).min().unwrap();
    println!("{}", m);
}


fn update(s: &mut BTreeMap<(i64, i64, i32), i64>, queue: &mut Vec<(i64, i64, i32)>, x: i64, y: i64, d: i32, new_score: i64) {

    match s.get_mut(&(x, y, d)) {
        Some(score) if *score > new_score => {
            *score = new_score;
            queue.push((x, y, d));
        },
        None => {
            s.insert((x, y, d), new_score);
            queue.push((x, y, d));
        },
        _ => {}
    }
}

fn get_from_map(x: i64, y: i64, map: &Vec<Vec<u8>>) -> Option<u8> {
    if let Ok(x) = usize::try_from(x) {
        if let Ok(y) = usize::try_from(y) {
            return map.get(x).and_then(|row| row.get(y).copied());
        }
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
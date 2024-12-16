use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string};
use std::ops::Bound::Included;

static DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

type Position = (i64, i64, i32);

fn main() {

    let map: Vec<Vec<u8>> = read_lines("input.txt").into_iter().map(|line| line.into_bytes()).collect();
    let (x, y, _) = map.iter().enumerate().flat_map(|(x, row)| row.iter().enumerate().filter(|b| *b.1 == b'S').map(move |(y, c)| (x as i64, y as i64, c))).next().unwrap();
    
    let mut s: BTreeMap<Position, (i64, Vec<Position>)> = BTreeMap::new();
    s.insert((x, y, 1), (0, Vec::new()));
    let mut queue: VecDeque<Position> = VecDeque::new();
    queue.push_back((x, y, 1));

    while let Some(p) = queue.pop_front() {
        let current_score = s.get(&p).unwrap().0;

        let (dx, dy) = DIRECTIONS[p.2 as usize];
        let (nx, ny) = (p.0 + dx, p.1 + dy);
        if get_from_map(nx, ny, &map).is_some_and(|u| u == b'.' || u == b'E') {
            update(&mut s, &mut queue, (nx, ny, p.2), current_score + 1, p);
        }

        update(&mut s, &mut queue, (p.0, p.1, (p.2-1).rem_euclid(4)), current_score + 1000, p);
        update(&mut s, &mut queue, (p.0, p.1, (p.2+1).rem_euclid(4)), current_score + 1000, p);
    }

    let (ex, ey, _) = map.iter().enumerate().flat_map(|(x, row)| row.iter().enumerate().filter(|b| *b.1 == b'E').map(move |(y, c)| (x as i64, y as i64, c))).next().unwrap();
    for d in 0..4 {
        let p = (ex, ey, d);
        if let Some(&(score, _)) = s.get(&p) {
            update(&mut s, &mut queue, (ex, ey, 4), score, p);
        }
    }
    println!("{}", count(&s, (ex, ey, 4)));
}

fn count(s: &BTreeMap<Position, (i64, Vec<Position>)>, p: Position) -> usize {
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(p);
    visited.insert(p);
    while let Some(p) = queue.pop_front() {
        for np in s[&p].1.iter() {
            if !visited.contains(np) {
                visited.insert(*np);
                queue.push_back(*np);
            }
        }
    }
    let spots: BTreeSet<(i64, i64)> = visited.iter().map(|p| (p.0, p.1)).collect();
    spots.len()
}

fn update(s: &mut BTreeMap<Position, (i64, Vec<Position>)>, queue: &mut VecDeque<Position>, p: Position, new_score: i64, from: Position) {
    match s.get_mut(&p) {
        Some(score) if score.0 > new_score => {
            *score = (new_score, vec![from; 1]);
            queue.push_back(p);
            //println!("{} {} {} {}", p.0, p.1, p.2, new_score);
        },
        Some(score) if score.0 == new_score => {
            score.1.push(from);
        },
        None => {
            s.insert(p, (new_score, vec![from; 1]));
            queue.push_back(p);
            //println!("{} {} {} {}", p.0, p.1, p.2, new_score);
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
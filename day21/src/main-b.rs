use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string, i64, iter};

use regex::Regex;

const N_D_PADS: usize = 25;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DKey {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    A
}

static DIRECTIONS: &'static [(i32, i32, DKey); 4] = &[(-1, 0, DKey::UP), (1, 0, DKey::DOWN), (0, -1, DKey::LEFT), (0, 1, DKey::RIGHT)]; 

type Coord = (i32, i32);

fn main() {
    let input: Vec<String> = read_lines("input.txt");
    
    let mut memory = BTreeMap::new();
    
    let pattern = Regex::new(r"^(\d+)A$").unwrap();
    let output: i64 = input.into_iter().map(|s| {
        let sequence: i64 = iter::once(&b'A').chain(s.as_bytes().iter())
            .zip(s.as_bytes().iter())
            .map(|(&from_num, &to_num)| {
                sch(n_pad(from_num), n_pad(to_num), DKey::A, 0, &mut memory) + 1
            })
            .sum();
        println!("");
        println!("{}", sequence);
        let num: i64 = pattern.captures(&s).unwrap()[1].parse().unwrap();
        println!("{}", num);
        sequence * num
    }).sum();

    println!("{}", output);
}

fn sch(from: Coord, to: Coord, current_d: DKey, level: usize, memory: &mut BTreeMap<(Coord, Coord, DKey, usize), i64>) -> i64 {
    if level == N_D_PADS {
        return ((from.0 - to.0).abs() + (from.1 - to.1).abs()) as i64;
    }

    if let Some(v) = memory.get(&(from, to, current_d, level)) {
        return *v;
    }
    
    if from == to {
        if current_d != DKey::A {
            let dist = sch(d_pad(current_d), d_pad(DKey::A), DKey::A, level + 1, memory);
            memory.insert((d_pad(current_d), d_pad(DKey::A), DKey::A, level + 1), dist);
            return dist;
        }
        else { return 0 };
    }
    
    // The real stuff. How can we enumerate all the paths between from and to?
    let mut min_dist = i64::MAX;
    for &(dx, dy, new_d) in DIRECTIONS {
        if dx.signum() != 0 && dx.signum() != (to.0 - from.0).signum() {
            continue;
        }
        if dy.signum() != 0 && dy.signum() != (to.1 - from.1).signum() {
            continue;
        }
        let new_coord = (from.0 + dx, from.1 + dy);
        if level == 0 {
            if new_coord.0 < 0 || new_coord.0 > 3 {
                continue;
            }
            if new_coord.1 < 0 || new_coord.1 > 2 {
                continue;
            }
            if new_coord.0 == 3 && new_coord.1 == 0 {
                continue;
            }
        } else {
            if new_coord.0 < 0 || new_coord.0 > 1 {
                continue;
            }
            if new_coord.1 < 0 || new_coord.1 > 2 {
                continue;
            }
            if new_coord.0 == 0 && new_coord.1 == 0 {
                continue;
            }
        }
        let turn = if new_d == current_d {
            0
        } else {
            sch(d_pad(current_d), d_pad(new_d), DKey::A, level + 1, memory)
        };
        let remaining = sch(new_coord, to, new_d, level, memory);
        // Turn + press + remaining
        let distance = turn + 1 + remaining;
        if distance < min_dist {
            min_dist = distance;
        }
    }
    
    memory.insert((from, to, current_d, level), min_dist);
    min_dist    
}

fn n_pad(key: u8) -> Coord {
    match key {
        b'7'=>(0, 0),
        b'8'=>(0, 1),
        b'9'=>(0, 2),
        b'4'=>(1, 0),
        b'5'=>(1, 1),
        b'6'=>(1, 2),
        b'1'=>(2, 0),
        b'2'=>(2, 1),
        b'3'=>(2, 2),
        b'0'=>(3, 1),
        b'A'=>(3, 2),
        _ => panic!()
    }
}

fn d_pad(key: DKey) -> Coord {
    match key {
        DKey::UP => (0, 1),
        DKey::RIGHT => (1, 2),
        DKey::DOWN => (1, 1),
        DKey::LEFT => (1, 0),
        DKey::A => (0, 2)
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
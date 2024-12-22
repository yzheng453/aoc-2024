use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fs::read_to_string, iter};

use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DKey {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    A
}

impl DKey {
    const VALUES: [Self; 5] = [Self::A, Self::DOWN, Self::LEFT, Self::RIGHT, Self::UP];
}

type Coord = (i32, i32);

type State = (u8, DKey, DKey);

#[derive(Clone, Copy)]
enum Path {
    State(State),
    Key(DKey)
}

fn main() {
    let input: Vec<String> = read_lines("input.txt");
    
    let (npad, npad_map) = num_pad();
    let dpad_map: BTreeMap<Coord, DKey> = DKey::VALUES.iter().map(|&d| (directional_pad(d), d)).collect();
    
    let mut dist: BTreeMap<(State, State), (i64, Path)> = BTreeMap::new();
    let states: Vec<State> = npad.keys().flat_map(|&n| 
        DKey::VALUES.iter().flat_map(move |&d1|
            DKey::VALUES.iter().map(move |&d2|
                (n, d1, d2)
            )
        )
    ).collect();
    
    for &(n, d1, d2) in states.iter() {
        // Move
        let c2 = directional_pad(d2);
        for d3 in DKey::VALUES {
            match d3 {
                DKey::UP    => if let Some(&d2n) = dpad_map.get(&(c2.0 - 1, c2.1)) { dist.insert(((n, d1, d2), (n, d1, d2n)), (1, Path::Key(d3))); }
                DKey::DOWN  => if let Some(&d2n) = dpad_map.get(&(c2.0 + 1, c2.1)) { dist.insert(((n, d1, d2), (n, d1, d2n)), (1, Path::Key(d3))); }
                DKey::LEFT  => if let Some(&d2n) = dpad_map.get(&(c2.0, c2.1 - 1)) { dist.insert(((n, d1, d2), (n, d1, d2n)), (1, Path::Key(d3))); }
                DKey::RIGHT => if let Some(&d2n) = dpad_map.get(&(c2.0, c2.1 + 1)) { dist.insert(((n, d1, d2), (n, d1, d2n)), (1, Path::Key(d3))); }
                _ => {}
            }
        }
        
        // Press
        let c1 = directional_pad(d1);
        match d2 {
            DKey::UP    => if let Some(&d1n) = dpad_map.get(&(c1.0 - 1, c1.1)) { dist.insert(((n, d1, d2), (n, d1n, d2)), (1, Path::Key(DKey::A))); }
            DKey::DOWN  => if let Some(&d1n) = dpad_map.get(&(c1.0 + 1, c1.1)) { dist.insert(((n, d1, d2), (n, d1n, d2)), (1, Path::Key(DKey::A))); }
            DKey::LEFT  => if let Some(&d1n) = dpad_map.get(&(c1.0, c1.1 - 1)) { dist.insert(((n, d1, d2), (n, d1n, d2)), (1, Path::Key(DKey::A))); }
            DKey::RIGHT => if let Some(&d1n) = dpad_map.get(&(c1.0, c1.1 + 1)) { dist.insert(((n, d1, d2), (n, d1n, d2)), (1, Path::Key(DKey::A))); }
            DKey::A => {
                let c = npad[&n];
                match d1 {
                    DKey::UP    => if let Some(&nn) = npad_map.get(&(c.0 - 1, c.1)) { dist.insert(((n, d1, d2), (nn, d1, d2)), (1, Path::Key(DKey::A))); }
                    DKey::DOWN  => if let Some(&nn) = npad_map.get(&(c.0 + 1, c.1)) { dist.insert(((n, d1, d2), (nn, d1, d2)), (1, Path::Key(DKey::A))); }
                    DKey::LEFT  => if let Some(&nn) = npad_map.get(&(c.0, c.1 - 1)) { dist.insert(((n, d1, d2), (nn, d1, d2)), (1, Path::Key(DKey::A))); }
                    DKey::RIGHT => if let Some(&nn) = npad_map.get(&(c.0, c.1 + 1)) { dist.insert(((n, d1, d2), (nn, d1, d2)), (1, Path::Key(DKey::A))); }
                    DKey::A => {
                        // Nothing
                    }
                }
            }
        }
    }
        
    for &k in states.iter() {
        for &i in states.iter() {
            for &j in states.iter() {
                if let (Some(&d1), Some(&d2)) = (dist.get(&(i, k)), dist.get(&(k, j))) {
                    dist.entry((i, j)).and_modify(|d| {
                        if d.0 > d1.0 + d2.0 {
                            *d = (d1.0 + d2.0, Path::State(k))
                        }
                    }).or_insert_with(|| (d1.0 + d2.0, Path::State(k)));
                }
            }
        }
    }
    
    
    let pattern = Regex::new(r"^(\d+)A$").unwrap();
    let output: i64 = input.into_iter().map(|s| {
        let sequence: i64 = iter::once(&b'A').chain(s.as_bytes().iter())
            .zip(s.as_bytes().iter())
            .map(|(&from_num, &to_num)| {
                let seq = &dist[&((from_num, DKey::A, DKey::A), (to_num, DKey::A, DKey::A))];               
                print_seq((from_num, DKey::A, DKey::A), (to_num, DKey::A, DKey::A), &dist);
                print!("A");
                seq.0 + 1
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

fn print_seq(from: State, to: State, dist: &BTreeMap<(State, State), (i64, Path)>) {
    let seq = &dist[&(from, to)];
    match seq.1 {
        Path::Key(c) => {
            print!("{}", match c {
                DKey::A => 'A',
                DKey::DOWN => 'v',
                DKey::LEFT => '<',
                DKey::RIGHT => '>',
                DKey::UP => '^',
            });                    
        },
        Path::State(mid) => {
            print_seq(from, mid, dist);
            print_seq(mid, to, dist);
        }
    }
}

fn num_pad() -> (BTreeMap<u8, Coord>, BTreeMap<Coord, u8>) {
    let mut p = BTreeMap::new();
    p.insert(b'7', (0, 0));
    p.insert(b'8', (0, 1));
    p.insert(b'9', (0, 2));
    p.insert(b'4', (1, 0));
    p.insert(b'5', (1, 1));
    p.insert(b'6', (1, 2));
    p.insert(b'1', (2, 0));
    p.insert(b'2', (2, 1));
    p.insert(b'3', (2, 2));
    p.insert(b'0', (3, 1));
    p.insert(b'A', (3, 2));
    
    let m: BTreeMap<Coord, u8> = p.iter().map(|(x, y)| (*y, *x)).collect();
    (p, m)
}

fn directional_pad(key: DKey) -> Coord {
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
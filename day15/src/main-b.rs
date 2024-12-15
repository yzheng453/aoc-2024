use std::{collections::{BTreeMap, BTreeSet}, fs::read_to_string};

static DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {

    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut instructions = Vec::new();
    let mut mode = 0;

    for line in read_lines("input.txt") {
        if line.len() == 0 {
            mode = 1;
        }
        match mode {
            0 => {
                map.push(line.into_bytes().into_iter().flat_map(|u| match u {
                    b'#' => [b'#', b'#'].into_iter(),
                    b'.' => [b'.', b'.'].into_iter(),
                    b'O' => [b'[', b']'].into_iter(),
                    b'@' => [b'@', b'.'].into_iter(),
                    _ => panic!()
                }).collect());
            },
            _ => {
                instructions.append(&mut line.into_bytes());
            }
        }
    }
    print_map(&map);
    let (mut x, mut y, _) = map.iter().enumerate().flat_map(|(x, row)| row.iter().enumerate().filter(|b| *b.1 == b'@').map(move |(y, c)| (x as i64, y as i64, c))).next().unwrap();
    map[x as usize][y as usize] = b'.';

    for i in instructions {
        let d = match i {
            b'^' => 0,
            b'>' => 1,
            b'v' => 2,
            b'<' => 3,
            _ => panic!()
        };
        let (dx, dy) = DIRECTIONS[d];
        let (nx, ny) = (x + dx, y + dy);
        if can_move(nx, ny, d, &map) {
            do_move(nx, ny, d, &mut map);
            (x, y) = (nx, ny);
        }
        println!("{} {}", x, y);
        //print_map(&map);
    }
    
    let sum: i64 = map.iter().enumerate().flat_map(|(x, row)| row.iter().enumerate().filter(|(_, u)| **u == b'[').map(move |(y, _)| 100 * (x as i64) + y as i64)).sum();
    println!("{}", sum);
}

fn print_map(map: &Vec<Vec<u8>>) {
    for row in map.iter() {
        for u in row {
            print!("{}", *u as char);
        }
        println!();
    }
}

fn can_move(x: i64, y: i64, d: usize, map: &Vec<Vec<u8>>) -> bool {
    return match get_from_map(x, y, map) {
        Some(b'#') | None => {
            false
        },
        Some(b'[') => {
            let (dx, dy) = DIRECTIONS[d];
            if d%2 == 0 {
                can_move(x + dx, y + dy, d, map) && can_move(x + dx, y + dy + 1, d, map)
            } else {
                can_move(x + dx, y + 2, d, map)
            }
        },
        Some(b']') => {
            let (dx, dy) = DIRECTIONS[d];
            if d%2 == 0 {
                can_move(x + dx, y + dy, d, map) && can_move(x + dx, y + dy - 1, d, map)
            } else {
                can_move(x + dx, y - 2, d, map)
            }
        },
        Some(b'.') => true,
        _ => panic!()
    }
} 

fn do_move(x: i64, y: i64, d: usize, map: &mut Vec<Vec<u8>>) {
    match get_from_map(x, y, map) {
        Some(b'[') => {
            let (dx, dy) = DIRECTIONS[d];
            if d%2 == 0 {
                do_move(x + dx, y + dy, d, map);
                do_move(x + dx, y + dy + 1, d, map);
                map[(x + dx) as usize][(y + dy) as usize] = b'[';
                map[(x + dx) as usize][(y + dy + 1) as usize] = b']';
                map[x as usize][y as usize] = b'.';
                map[x as usize][(y + 1) as usize] = b'.';
            } else {
                do_move(x + dx, y + 2, d, map);
                map[(x + dx) as usize][(y + dy) as usize] = b'[';
                map[(x + dx) as usize][(y + dy + 1) as usize] = b']';
                map[x as usize][y as usize] = b'.';
            }
        },
        Some(b']') => {
            let (dx, dy) = DIRECTIONS[d];
            if d%2 == 0 {
                do_move(x + dx, y + dy, d, map);
                do_move(x + dx, y + dy - 1, d, map);
                map[(x + dx) as usize][(y + dy - 1) as usize] = b'[';
                map[(x + dx) as usize][(y + dy) as usize] = b']';
                map[x as usize][y as usize] = b'.';
                map[x as usize][(y - 1) as usize] = b'.';
            } else {
                do_move(x + dx, y - 2, d, map);
                map[(x + dx) as usize][(y + dy - 1) as usize] = b'[';
                map[(x + dx) as usize][(y + dy) as usize] = b']';
                map[x as usize][y as usize] = b'.';
            }
        },
        Some(b'.') => {},
        _ => panic!()
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
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn search_x_mas(p: &Vec<Vec<u8>>) -> i64 {
    let n = p.len();
    let m = p[0].len();    
    let mut s = 0;
    for i in 0..n {
        for j in 0..m {
            if p.get(i).and_then(|row| row.get(j)).is_none_or(|&u| u != b'A') {
                continue;
            }
            /* Apparently the below one is a +-MAS, and doesn't count as a X-MAS. LOL
            if check_mas(&p, i, j, 1, 0) && check_mas(&p, i, j, 0, 1) {
                s += 1;
            }
            */

            if check_mas(&p, i, j, 1, 1) && check_mas(&p, i, j, 1, -1) {
                s += 1;
            }

        }
    }
    s
}

fn check_mas(p: &Vec<Vec<u8>>, x: usize, y: usize, dx: i64, dy: i64) -> bool {
    if let Some(&b1) = get_byte(p, x as i64 + dx, y as i64 + dy) {
        if let Some(&b2) = get_byte(p, x as i64 - dx, y as i64 - dy) {
            if (b1 == b'M' || b1 == b'S') && (b2 == b'M' || b2 == b'S') && (b1 != b2) {
                return true;
            }
        }
    }
    return false;
}

fn get_byte(p: &Vec<Vec<u8>>, x: i64 , y: i64) -> Option<&u8> {
    usize::try_from(x).and_then(|x1| usize::try_from(y).map(|y1| (x1, y1)))
        .ok()
        .and_then(|(x1, y1)|
            p.get(x1).and_then(|row| row.get(y1))
        )
}

fn main() {
    
    let p: Vec<Vec<u8>> = read_lines("input.txt").into_iter()
        .map(|s| s.into_bytes()).collect();
    
    let sum: i64 = search_x_mas(&p);

    println!("{}", sum);

}


use itertools::Itertools;
use std::fs;

pub fn run() {
    let content = fs::read_to_string("./05-stacks.moves").unwrap();
    let res = move_around(content.clone()).unwrap();
    println!("Result: {}", res);
    let res = move_around_two(content.clone()).unwrap();
    println!("Result: {}", res);
}


fn move_around(content: String) -> Option<String> {
    let s1 = vec!['G', 'T', 'R', 'W'];
    let s2 = vec!['G', 'C', 'H', 'P', 'M', 'S', 'V', 'W'];
    let s3 = vec!['C', 'L', 'T', 'S', 'G', 'M'];
    let s4 = vec!['J', 'H', 'D', 'M', 'W', 'R', 'F'];
    let s5 = vec!['P', 'Q', 'L', 'H', 'S', 'W', 'F', 'J'];
    let s6 = vec!['P', 'J', 'D', 'N', 'F', 'M', 'S'];
    let s7 = vec!['Z', 'B', 'D', 'F', 'G', 'C', 'S', 'J'];
    let s8 = vec!['R', 'T', 'B'];
    let s9 = vec!['H', 'N', 'W', 'L', 'C'];
    let mut s = [s1, s2, s3, s4, s5, s6, s7, s8, s9];
    for mov in content.lines() {
        let (_, n, _, from, _, to) = mov.split(' ').next_tuple().unwrap();
        let n = n.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap();
        let to = to.parse::<usize>().unwrap();
        for _ in 0..n {
            let e = s[from - 1].pop().unwrap();
            s[to - 1].push(e);
        }
    }
    let mut result = String::new();
    for mut l in s {
        result.push(l.pop().unwrap());
    }
    Some(result)
}

fn move_around_two(content: String) -> Option<String> {
    let s1 = vec!['G', 'T', 'R', 'W'];
    let s2 = vec!['G', 'C', 'H', 'P', 'M', 'S', 'V', 'W'];
    let s3 = vec!['C', 'L', 'T', 'S', 'G', 'M'];
    let s4 = vec!['J', 'H', 'D', 'M', 'W', 'R', 'F'];
    let s5 = vec!['P', 'Q', 'L', 'H', 'S', 'W', 'F', 'J'];
    let s6 = vec!['P', 'J', 'D', 'N', 'F', 'M', 'S'];
    let s7 = vec!['Z', 'B', 'D', 'F', 'G', 'C', 'S', 'J'];
    let s8 = vec!['R', 'T', 'B'];
    let s9 = vec!['H', 'N', 'W', 'L', 'C'];
    let mut s = [s1, s2, s3, s4, s5, s6, s7, s8, s9];
    for mov in content.lines() {
        let (_, n, _, from, _, to) = mov.split(' ').next_tuple().unwrap();
        let n = n.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap();
        let to = to.parse::<usize>().unwrap();
        let mut v: Vec<char> = Vec::new();
        for _ in 0..n {
            let e = s[from - 1].pop().unwrap();
            v.push(e);
        }
        while !v.is_empty() {
            s[to - 1].push(v.pop().unwrap());
        }
        v.clear();
    }
    let mut result = String::new();
    for mut l in s {
        result.push(l.pop().unwrap());
    }
    Some(result)
}

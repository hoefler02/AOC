use std::collections::HashSet;
use std::hash::Hash;
use std::fs;

pub fn run() {
    let content = fs::read_to_string("./06-marker.input").unwrap();
    println!("Part One: {}", find_marker(content.clone()).unwrap());
    println!("Part Two: {}", find_start(content.clone()).unwrap());
}

fn find_marker(content: String) -> Option<u32> {
    let mut inp = content.chars();
    let mut a = inp.next().unwrap();
    let mut b = inp.next().unwrap();
    let mut c = inp.next().unwrap();
    let mut res = 0;
    while let Some(d) = inp.next() {
        if d != a && d != b && d != c && a != b && a != c && b != c{
            break;
        }
        a = b;
        b = c;
        c = d;
        res += 1;
    }
    Some(res + 4)
}

fn find_start(content: String) -> Option<u32> {
    let mut inp = content.as_bytes().to_vec();
    let mut a = 0;
    let mut b = 14;
    while !has_unique_elements(&inp[a..b]) {
        a += 1;
        b += 1;
    }
    Some(b as u32)
}

// https://stackoverflow.com/questions/46766560/how-to-check-if-there-are-duplicates-in-a-slice
fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

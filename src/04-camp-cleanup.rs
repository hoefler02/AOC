use std::fs;

pub fn run() {
    // read in our input file
    let path = "04-camp-cleanup.input";
    let content = fs::read_to_string(path).unwrap();
    let a = part_one_overlaps(content.clone()).unwrap();
    println!("Part One Overlaps: {}", a);
    let a = part_two_overlaps(content.clone()).unwrap();
    println!("Part Two Overlaps: {}", a);
}

// let pub part_one_check = |u1: u32, u2: u32, v1: u32, v2: u32| -> u32 {
//         (v1 >= u1 && v1 <= u2) && (v2 <= u2 && v2 >= v1) ||
//         (u1 >= v1 && u1 <= v2) && (u2 <= v2 && u2 >= u1)
// };

fn part_one_overlaps(content: String) -> Option<u32> {
    let mut n: u32 = 0;
    for line in content.lines() {
        let mut is = line.split(',');
        let (i1, i2) = (is.next().unwrap(), is.next().unwrap());
        let (mut i1, mut i2) = (i1.split('-'), i2.split('-'));
        let (u1, u2) = (i1.next().unwrap().parse::<u32>().unwrap(), i1.next().unwrap().parse::<u32>().unwrap());
        let (v1, v2) = (i2.next().unwrap().parse::<u32>().unwrap(), i2.next().unwrap().parse::<u32>().unwrap());
        if (v1 >= u1 && v1 <= u2) && (v2 <= u2 && v2 >= v1) {
            n += 1;
            continue;
        }
        if (u1 >= v1 && u1 <= v2) && (u2 <= v2 && u2 >= u1) {
            n += 1;
            continue;
        }
    }
    Some(n)
}

fn part_two_overlaps(content: String) -> Option<u32> {
    let mut n: u32 = 0;
    for line in content.lines() {
        let mut is = line.split(',');
        let (i1, i2) = (is.next().unwrap(), is.next().unwrap());
        let (mut i1, mut i2) = (i1.split('-'), i2.split('-'));
        let (u1, u2) = (i1.next().unwrap().parse::<u32>().unwrap(), i1.next().unwrap().parse::<u32>().unwrap());
        let (v1, v2) = (i2.next().unwrap().parse::<u32>().unwrap(), i2.next().unwrap().parse::<u32>().unwrap());
        if (v1 >= u1 && v1 <= u2) || (v2 <= u2 && v2 >= u1) {
            n += 1;
            continue;
        }
        if (u1 >= v1 && u1 <= v2) || (u2 <= v2 && u2 >= v1) {
            n += 1;
            continue;
        }
    }
    Some(n)
}

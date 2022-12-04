use std::fs;

const ITEMS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z', 'A', 'B', 'C', 'D',
	'E', 'F', 'G', 'H', 'I',
	'J', 'K', 'L', 'M', 'N', 
	'O', 'P', 'Q', 'R', 'S', 
	'T', 'U', 'V', 'W', 'X',
	'Y', 'Z'
];


pub fn run() {
    // read in our input file
    let path = "03-rucksack.input";
    let content = fs::read_to_string(path).unwrap();
	let p = part_one_priorities(content.clone()).unwrap();
	println!("Part One: {}", p);
	let p = part_two_priorities(content.clone()).unwrap();
	println!("Part Two: {}", p);
}

fn part_one_priorities(content: String) -> Option<u32> {
	let mut s: u32 = 0;
    for line in content.lines() {
        let clen = line.len() / 2;
        if clen > 0 {
            // split into individual compartments
            let (c1, c2) = line.split_at(clen);
            let c1: Vec<char> = c1.chars().collect();
            let c2: Vec<char> = c2.chars().collect();
			for e in ITEMS.iter().enumerate() {
				if c1.contains(e.1) && c2.contains(e.1) {
					s += (e.0 as u32) + 1;
				}
	        }		
        }
    }
	Some(s)
}

fn part_two_priorities(content: String) -> Option<u32> {
    let mut lines = content.lines();
    let mut s = 0;
    // grab three lines
    while let (Some(l1), Some(l2), Some(l3)) = (lines.next(), lines.next(), lines.next()) {
		for e in ITEMS.iter().enumerate() {
			if l1.contains(*e.1) && l2.contains(*e.1) && l3.contains(*e.1) {
				s += (e.0 as u32) + 1;
			}
        }		
    }
    Some(s)
}



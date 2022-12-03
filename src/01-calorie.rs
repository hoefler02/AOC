use std::process;
use std::fs;

pub fn run() {

    let path = "./01-calorie.input";
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            println!("File Error: {}", e);
            process::exit(1);
        }
    };

    // grab each line of the file
    let lines = content.lines();

    // variables for maxs and current sums (part II)
    let mut maxs: Vec<u32> = vec![0, 0, 0];
    let mut curr: u32 = 0;

    // iterate through the lines and calculate sums
    for line in lines {
        if !line.is_empty() {
            curr += line.parse::<u32>().expect("Not a Number");
        } else {
            let min = maxs.iter_mut().min().unwrap();
            // check if we found a new max
            if curr > *min {
                *min = curr;
            }
            // reset the current sum
            curr = 0;
        }
    }

    // largest is first challenge
    println!("Part One: {:?}", maxs.iter().max().unwrap());
    // second challenge
    println!("Part Two: {}", maxs.iter().sum::<u32>());

}


use std::process;
use std::env;
use std::fs;

fn main() {

    // read the file in as an argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} [input]", args[0]);
        process::exit(1);
    }
    let path = args[1].clone();
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
    println!("Max Arr: {:?}", maxs);
    // second challenge
    println!("Max Sum: {}", maxs.iter().sum::<u32>());

}


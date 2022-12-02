use std::process;
use std::env;
use std::fs;

fn main() {
    // check our arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [input]", args[0]);
        process::exit(1);
    }
    let path = args[1].clone();
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Issue Opening File: {:?}", e);
            process::exit(1);
        }
    };
    let s = part_one_score(content.clone()).unwrap();
    println!("Part One Score: {}", s);
    let s = part_two_score(content).unwrap();
    println!("Part Two Score: {}", s);
}

// paper beats rock
// rock beats scissors
// scissors beats paper

#[derive(Debug, Clone, PartialEq, Eq)]
enum Game {
    Rock, // 0
    Paper, // 1
    Scissors // 2
}


fn part_one_score(games: String) -> Option<u32> {
    let mut s = 0;
    for game in games.lines() {
        let mut moves = game.split_whitespace();
        // probably not the best way to get the first chars :)
        let theirs = letter_to_move(moves.next().unwrap_or_default().chars().next().unwrap_or_default());
        let ours = letter_to_move(moves.next().unwrap_or_default().chars().next().unwrap_or_default());
        if ours.is_none() || theirs.is_none() {
            eprintln!("File Format Error!");
            process::exit(1);
        }
        let ours = ours.unwrap();
        let theirs = theirs.unwrap();
        // add the move value
        s += (ours.clone() as u32) + 1;
        // if we have a tie
        if ours == theirs {
            s += 3;
        } 
        // if we have a win
        if (ours == Game::Paper && theirs == Game::Rock) ||
           (ours == Game::Scissors && theirs == Game::Paper) ||
           (ours == Game::Rock && theirs == Game::Scissors) {
               s += 6;
        }
    }
    Some(s)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Outcome {
    Loss,
    Tie,
    Win
}

fn part_two_score(games: String) -> Option<u32> {
    let mut s = 0;
    for game in games.lines() {
        let mut moves = game.split_whitespace();
        // probably not the best way to get the first chars :)
        let theirs = letter_to_move(moves.next().unwrap_or_default().chars().next().unwrap_or_default());
        let outcome = letter_to_outcome(moves.next().unwrap_or_default().chars().next().unwrap_or_default());
        if outcome.is_none() || theirs.is_none() {
            eprintln!("File Format Error!");
            process::exit(1);
        }
        let outcome = outcome.unwrap();
        let theirs = theirs.unwrap();
        // add the win/tie/loss value
        s += (outcome.clone() as u32) * 3;
        if (outcome == Outcome::Loss && theirs == Game::Paper) ||
           (outcome == Outcome::Tie  && theirs == Game::Rock)  ||
           (outcome == Outcome::Win  && theirs == Game::Scissors) {
               s += 1;
        } else if (outcome == Outcome::Loss && theirs == Game::Scissors) ||
                  (outcome == Outcome::Tie && theirs == Game::Paper) ||
                  (outcome == Outcome::Win && theirs == Game::Rock) {
                s += 2;
        } else {
                s += 3;
        }
    }
    Some(s)
}

fn letter_to_move(letter: char) -> Option<Game> {
    match letter {
        'A' => Some(Game::Rock),
        'B' => Some(Game::Paper),
        'C' => Some(Game::Scissors),
        'X' => Some(Game::Rock),
        'Y' => Some(Game::Paper),
        'Z' => Some(Game::Scissors),
         _  => None
    }
}


fn letter_to_outcome(letter: char) -> Option<Outcome> {
    match letter {
        'X' => Some(Outcome::Loss),
        'Y' => Some(Outcome::Tie),
        'Z' => Some(Outcome::Win),
         _  => None
    }
}


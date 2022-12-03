use std::process;
use std::fs;

pub fn run() {
    let path = "02-rock-paper-scissors.input";
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Issue Opening File: {:?}", e);
            process::exit(1);
        }
    };
    let s = part_one_score(content.clone()).unwrap();
    println!("Part One: {}", s);
    let s = part_two_score(content.clone()).unwrap();
    println!("Part Two: {}", s);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}
#[derive(Debug, Clone, PartialEq, Eq)] enum Outcome {
    Loss,
    Tie,
    Win
}

fn get_move(letter: char) -> Option<Move> {
    match letter {
        'A' | 'X' => Some(Move::Rock),
        'B' | 'Y' => Some(Move::Paper),
        'C' | 'Z' => Some(Move::Scissors),
         _  => None
    }
}


fn get_outcome(letter: char) -> Option<Outcome> {
    match letter {
        'X' => Some(Outcome::Loss),
        'Y' => Some(Outcome::Tie),
        'Z' => Some(Outcome::Win),
         _  => None
    }
}

fn part_one_score(games: String) -> Option<u32> {
    let mut s = 0;
    // cycle through the games
    for game in games.lines() {
        // remove the spaces
        let mut moves = game.chars().filter(|c| !c.is_whitespace());
        let m1 = get_move(moves.next()?)?; // their move
        let m2 = get_move(moves.next()?)?; // our move
        // add the move value
        s += m2.clone() as u32;
        // if we have a tie
        if m1 == m2 {
            s += 3;
        } 
        // if we have a win
        if (m2 == Move::Paper && m1 == Move::Rock) ||
           (m2 == Move::Scissors && m1 == Move::Paper) ||
           (m2 == Move::Rock && m1 == Move::Scissors) {
               s += 6;
        }
    }
    Some(s)
}


fn part_two_score(games: String) -> Option<u32> {
    let mut s = 0;
    for game in games.lines() {
        let mut moves = game.chars().filter(|c| !c.is_whitespace());
        let m = get_move(moves.next()?)?;
        let o = get_outcome(moves.next()?)?;
        // add the win/tie/loss value
        s += (o.clone() as u32) * 3;
        // manually checking the move (9 basic cases)
        if (o == Outcome::Loss && m == Move::Paper) ||
           (o == Outcome::Tie  && m == Move::Rock)  ||
           (o == Outcome::Win  && m == Move::Scissors) {
                s += 1;
        } else if (o == Outcome::Loss  &&  m == Move::Scissors) ||
                  (o == Outcome::Tie   &&  m == Move::Paper) ||
                  (o == Outcome::Win   &&  m == Move::Rock) {
                s += 2;
        } else {
                s += 3;
        }
    }
    Some(s)
}



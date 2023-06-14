use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Eq, PartialEq)]
enum Sign {
    Rock,
    Paper,
    Scissors
}
enum WinState {
    You,
    Opponent,
    Draw,
}
#[allow(dead_code)]
impl Sign {
    fn new(input:&char) -> Option<Self> {
        match input {
            'A' | 'X' => Some(Self::Rock),
            'B' | 'Y' => Some(Self::Paper),
            'C' | 'Z' => Some(Self::Scissors),
            _ => None
        }
    }
    fn to_score(&self) -> i32 {
        match self {
            Sign::Rock => 1,
            Sign::Paper => 2,
            Sign::Scissors => 3,
        }
    }
}
#[allow(dead_code)]
impl WinState {
    fn get(you:&Sign, opponent:&Sign) -> WinState {
        if you == opponent  {
            return WinState::Draw;
        }
        match (you, opponent) {
            (Sign::Rock, Sign::Scissors) |
            (Sign::Scissors, Sign::Paper) |
            (Sign::Paper, Sign::Rock)
            => WinState::You,
            (_, _) => WinState::Opponent,
        }
        
    }
    fn to_score(&self) -> i32 {
        match self {
            WinState::You => 6,
            WinState::Draw => 3,
            WinState::Opponent => 0,
        }
    }
}

fn main()  {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    
    let mut score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut chars = line.chars();
        let opponent = Sign::new(&chars.next().unwrap()).unwrap();
        chars.next(); // skip whitespace
        let you = Sign::new(&chars.next().unwrap()).unwrap();

        let win_state = WinState::get(&you, &opponent);

        score += win_state.to_score() + you.to_score();
    }

    
    println!("{}", score);
}
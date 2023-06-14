use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Eq, PartialEq, Clone)]
enum Sign {
    Rock,
    Paper,
    Scissors
}
#[derive(Eq, PartialEq)]
enum WinState {
    You,
    Opponent,
    Draw,
}

impl Sign {
    fn get_opponent(input:&char) -> Option<Self> {
        match input {
            'A' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'C' => Some(Self::Scissors),
            _ => None
        }
    }
    fn get_you(input:&char, opponent:Sign) -> Self {
        let desired_win_state = match input {
            'X' => Some(WinState::Opponent),
            'Y' => Some(WinState::Draw),
            'Z' => Some(WinState::You),
            _ => None
        }.unwrap();

        if desired_win_state == WinState::Draw {
            return opponent;
        }
        else if desired_win_state == WinState::You {
            match opponent {
                Sign::Rock => return Sign::Paper,
                Sign::Paper => return Sign::Scissors,
                Sign::Scissors => return Sign::Rock,
            }
        }
        else
        {
            match opponent {
                Sign::Rock => return Sign::Scissors,
                Sign::Paper => return Sign::Rock,
                Sign::Scissors => return Sign::Paper,
            }
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
        let opponent = Sign::get_opponent(&chars.next().unwrap()).unwrap();
        chars.next(); // skip whitespace
        let you = Sign::get_you(&chars.next().unwrap(), opponent.clone());

        let win_state = WinState::get(&you, &opponent);

        score += win_state.to_score() + you.to_score();
    }

    
    println!("{}", score);
}
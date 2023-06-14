use std::{fs::File, io::{BufReader, BufRead}};

fn main()  {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (first, second) = line.split_at(line.len()/2);
        let shared = first.chars()
            .into_iter()
            .find(|a| {
                second.chars().into_iter().find(|b| b == a).is_some()
            }).unwrap();
        score += match shared.is_uppercase() {
            true => shared as u32 - 'A' as u32 + 27,
            false => shared as u32 - 'a' as u32 + 1,
        };
    }
    println!("{}", score);

}
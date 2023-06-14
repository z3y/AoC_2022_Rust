use std::{fs::File, io::{BufReader, BufRead}};
fn main()  {
    let file = File::open("input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut score = 0;
    while let Some(first) = lines.next() {
        let first = first.unwrap();
        let second = lines.next().unwrap().unwrap();
        let third = lines.next().unwrap().unwrap();

        let shared = first.chars().into_iter().find(|a| {
                second.chars().into_iter().find(|b| {
                    third.chars().into_iter().find(|c| c == b && c == a).is_some()
                }).is_some()
            }).unwrap();
        score += match shared.is_uppercase() {
            true => shared as u32 - 'A' as u32 + 27,
            false => shared as u32 - 'a' as u32 + 1,
        }
    }

    println!("{}", score);

}
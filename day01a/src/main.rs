use std::{fs::File, io::{BufReader, BufRead}, cmp::max};

fn main()  {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    
    let mut most_calories = 0;
    let mut current_calories = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            most_calories = max(most_calories, current_calories);
            current_calories = 0;
            continue;
        }

        current_calories += line.parse::<i32>().unwrap();
    }

    
    println!("{}", most_calories);
}
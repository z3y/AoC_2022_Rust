use std::{fs::File, io::{BufReader, BufRead}, cmp::max, vec};

fn main()  {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut most_calories = vec![0; 3];
    let mut current_calories = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            let x = most_calories.iter_mut().min().unwrap();
            *x = max(current_calories, *x);
            current_calories = 0;
            continue;
        }

        current_calories += line.parse::<i32>().unwrap();
    }
    
    println!("{}", most_calories.iter().sum::<i32>());
}
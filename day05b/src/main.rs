use std::{fs::File, io::{BufReader, BufRead}};


fn main()  {
    let file = File::open("input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut rows: Vec<Vec<char>> = vec![vec![]; 9];

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.chars().nth(1).unwrap() == '1' {
            lines.next();
            break
        }
        
        let mut offset = 0;
        let mut index = 0;
        loop {
            let start = offset;
            let end = 3 + offset;
            if end > line.len() {
                break;
            }
            let mut current_crate = line[start..end].chars();
            let crate_is_empty = current_crate.all(|x| x == ' ');

            if !crate_is_empty {
                let prased = current_crate.nth(0).unwrap();
                rows[index].push(prased);
            }
            index +=1;
            offset +=4;
        }
    }

    rows.iter().for_each(|row| {
        row.iter().for_each(|column| {
            print!("[{}] ", column)
        });
        println!(" ")
    });

    rows.iter_mut().for_each(|x| x.reverse());
    println!();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.is_empty() { break }
        let mut split = line.split(' ');
        split.next();
        let move_count = split.next().unwrap().parse::<usize>().unwrap();
        split.next();
        let from_pos = split.next().unwrap().parse::<usize>().unwrap() - 1;
        split.next();
        let to_pos = split.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut i: usize = move_count;
        while i > 0 {
            let top_crate_val = rows[from_pos][rows[from_pos].len()-i];
            rows[to_pos].push(top_crate_val);
            //println!("moving {} from {} to {}", move_count, from_pos, to_pos);
            i-=1;
        }
        i = 0;
        while i < move_count {
            rows[from_pos].pop();
            i+=1;
        }

    }

    rows.iter().for_each(|row| {
        row.iter().for_each(|column| {
            print!("[{}] ", column)
        });
        println!(" ")
    });

    println!();
    rows.iter().for_each(|x| print!("{}", x.iter().last().unwrap()));
    println!();


}
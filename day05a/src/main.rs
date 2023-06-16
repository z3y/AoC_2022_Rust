use std::{fs::File, io::{BufReader, BufRead}};

fn main()  {
    let file = File::open("input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut rows: Vec<Vec<Option<char>>> = vec![vec![]; 9];
    let mut row = 0;

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.is_empty() { break }
        
        let mut offset = 0;
        loop {
            let start = offset;
            let end = 3 + offset;
            if end > line.len() {
                row+=1;
                break;
            }
            let mut current_crate = line[start..end].chars();
            let crate_is_empty = current_crate.all(|x| x == ' ');

            if crate_is_empty {
                rows[row].push(None);
            }
            else {
                let prased = current_crate.nth(0).unwrap();
                rows[row].push(Some(prased));
            }

            offset +=4;
        }
    }

//    rows.iter_mut().for_each(|x| x.reverse());

    for row in rows {
        for column in row {
            match column {
                Some(x) => print!("[{}] ", x),
                None => print!("    "),
            }
        }
        println!(" ")
    }

    while let Some(line) = lines.next() {
        let line = line.unwrap();

    }

    // println!("{}", num);

}
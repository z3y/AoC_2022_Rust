use std::{fs::File, io::{BufReader, BufRead}, cmp};

fn main()  {
    let file = File::open("input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut num = 0;
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut split = line.split(',');
        let (lhs, rhs) = (split.next().unwrap(), split.next().unwrap());

        let mut split = lhs.split('-');
        let (a0, a1) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());

        let mut split = rhs.split('-');
        let (b0, b1) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());

        if cmp::min(b0, b1) <= cmp::max(a0, a1)
        && cmp::max(b0, b1) >= cmp::min(a0, a1) {
            num+=1;
        }
    }

    println!("{}", num);

}
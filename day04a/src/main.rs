use std::{fs::File, io::{BufReader, BufRead}};

fn main()  {
    let file = File::open("input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut num = 0;
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut split = line.split(',');
        let (lhs, rhs) = (split.next().unwrap(), split.next().unwrap());

        let mut split = lhs.split('-');
        let (r00, r01) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());

        let mut split = rhs.split('-');
        let (r10, r11) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());

        if r00 >= r10 && r01 <= r11 {
            num+=1;
        }
        else if r00 <= r10 && r01 >= r11{
            num+=1;
        }
    }

    println!("{}", num);

}
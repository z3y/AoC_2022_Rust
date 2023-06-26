use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32
}
fn move_position(position: &mut Position, direction: u8) {

    match direction {
        b'L' => { position.x -= 1 },
        b'R' => { position.x += 1 },
        b'U' => { position.y += 1 },
        b'D' => { position.y -= 1 },
        _ => panic!()
    }
}

fn is_near_head(head: &Position, tail: &Position) -> bool {
    let mut x_touching = false;
    let mut y_touching = false;
    if head.x == tail.x || head.x + 1 == tail.x || head.x - 1 == tail.x {
        x_touching = true;
    }

    if head.y == tail.y || head.y + 1 == tail.y || head.y - 1 == tail.y {
        y_touching = true;
    }
    x_touching && y_touching 
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut visited_positions = HashSet::new();
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };

    visited_positions.insert(tail_position);

    for line in lines {
        let line = line.unwrap();
        let mut split = line.split_ascii_whitespace();
        let direction = split.next().unwrap().as_bytes()[0];
        let length = split.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..length {
            let previous_head = head_position.clone();
            move_position(&mut head_position, direction);

            if !is_near_head(&head_position, &tail_position) {
                tail_position = previous_head;
                visited_positions.insert(tail_position);
            }
        }
    }
    println!("Unique Positions {}", visited_positions.len());
}

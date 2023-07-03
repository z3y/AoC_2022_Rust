use std::{fs::File, io::{BufReader, BufRead}, cmp::max};

struct Talbe {
    x: i32,
    y: i32,
    size: i32,
    input: Vec<Vec<i32>>
}
enum MoveDirection {
    Up,
    Down,
    Left,
    Right
}
impl MoveDirection {
    fn from_int(value: i32) -> Self {
        match value {
            0 => MoveDirection::Up,
            1 => MoveDirection::Down,
            2 => MoveDirection::Left,
            3 => MoveDirection::Right,
            _ => panic!(),
        }
    }
}
impl Talbe {
        
    fn check_bounds(&self) -> bool {
        self.x >= 0 && self.x < self.size && self.y >= 0 && self.y < self.size
    }

    fn move_cell(&mut self, direction: &MoveDirection) -> Option<i32> {
        match direction {
            MoveDirection::Up => self.x -= 1,
            MoveDirection::Down => self.x += 1,
            MoveDirection::Left => self.y -= 1,
            MoveDirection::Right => self.y += 1,
        }
        match self.check_bounds() {
            true => Some(self.value_at_position()),
            false => None,
        }
    }

    fn value_at_position(&self) -> i32 {
        self.input[self.x as usize][self.y as usize]
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut table: Vec<Vec<i32>> = vec![];
    lines.into_iter().for_each(|x| {
        let mut vec = vec![];
        x.unwrap().chars().into_iter().for_each(|x| vec.push(x.to_digit(10).unwrap() as i32));
        table.push(vec);
    });
    let size = table.len() as i32;
    let mut table = Talbe { x: 0, y: 0, size, input: table };

    let mut max_scenic_score= 0;
    for x in 0..table.size {
        for y in 0..table.size {

            let mut view_distances = [0; 4];
            for i in 0..4 {
                let dir = MoveDirection::from_int(i);
                table.x = x;
                table.y = y;
                let current_tree_height = table.value_at_position();
                loop {
                    match table.move_cell(&dir) {
                        Some(tree) => {
                            if tree >= current_tree_height {
                                view_distances[i as usize] += 1;
                                break;
                            }
                        },
                        None => break
                    }
                    view_distances[i as usize] += 1;
                }
            }
            
            let scenic_score = view_distances.into_iter().product::<i32>();
            max_scenic_score = max(scenic_score, max_scenic_score);
        }
    }

    println!("{:?}", max_scenic_score);
}

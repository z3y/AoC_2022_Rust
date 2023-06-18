use std::{fs::File, io::{BufReader, BufRead}, collections::{LinkedList, HashSet}};

fn check_marker(list:&mut LinkedList<char>) -> bool {
    let mut hash_set = HashSet::new();
    list.iter().for_each(|x| _ = hash_set.insert(x));
    hash_set.len() == 4
}

fn main()  {
    let file = File::open("input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let line = lines.next().unwrap().unwrap();
    let chars_input = line.chars();
    let mut char_list = LinkedList::new();

    for (i, char) in chars_input.enumerate() {
        char_list.push_front(char);

        if char_list.len() == 4 {
            if check_marker(&mut char_list) {
                println!("{}", i+1);
                break;
            }
            char_list.pop_back();
        }
    }


}
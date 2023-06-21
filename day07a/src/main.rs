use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug)]
enum Node {
    Folder {
        name: String,
        children: Vec<usize>
    },
    File {
        name: String,
        size: i32
    }
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Node>,
    parents: Vec<usize>,
}
impl Tree {
    pub fn insert(&mut self, node: Node, parent: usize) -> usize {
        self.nodes.push(node);
        self.parents.push(parent);
        let node_idx = self.nodes.len();
        match &mut self.nodes[parent] {
            Node::Folder { name, children } => children.push(node_idx),
            Node::File { name, size } => (),
        }
        node_idx
    }
    pub fn get_parent(&self, node: usize) -> usize {
        self.parents[node]
    }
}
impl Node {
    pub fn new_folder(name: &str) -> Self {
        Node::Folder { name: String::from(name), children: vec![] }
    }
    pub fn new_file(name: &str, size: i32) -> Self {
        Node::File { name: String::from(name), size }
    }
}

#[derive(Debug)]
enum TokenKind {
    Ls,
    Cd(String),
    None,
}

fn main()  {
    let file = File::open("input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut tree = Tree { nodes: vec![], parents: vec![] };
    let root = Node::new_folder("/");
    let root = tree.insert(root, 0);
    let current_folder = root;

    let mut token = TokenKind::None;
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        // let mut chars = line.chars();
        let mut split = line.split(' ');
        let first_arg = split.next().unwrap();
        
        // println!("{:?}", token);
        
        if first_arg == "$"
        {
            let command_type = split.next().unwrap();

            match command_type {
                "ls" => { token = TokenKind::Ls; continue }
                "cd" =>  { token = TokenKind::Cd(split.next().unwrap().to_owned()); continue },
                _ => ()
            }
        }

        match token {
            TokenKind::Ls => {
                if first_arg == "dir" {
                    let folder = Node::new_folder(split.next().unwrap());
                    let root = tree.insert(folder, current_folder);
                }
                else {
                    let size = split.next().unwrap().parse::<i32>().unwrap();
                    current_folder.add_file(SystemFile::new(split.next().unwrap().to_owned(), size));
                }
                continue;
            }
            TokenKind::Cd(path) => {
                if path == ".." {
                    current_folder = current_folder.get_parent();
                }
                else if path == "/" {
                    current_folder = &mut root;
                }
                else {
                    current_folder = &mut current_folder.get_folder(path);
                }
            },
            TokenKind::None => ()
        }


    }

    println!("{:#?}", tree);
}
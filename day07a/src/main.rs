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
            Node::Folder { name: _, children } => { 
                // println!("adding child {}", node_idx);
                children.push(node_idx)},
            Node::File { name: _, size: _ } => (),
        }
        node_idx
    }
    pub fn get_parent(&self, node: usize) -> usize {
        self.parents[node]
    }
    pub fn get_children(&self, folder: usize) -> Vec<usize> {
        match &self.nodes[folder] {
            Node::Folder { name: _, children } => children.to_vec(),
            Node::File { name: _, size: _ } => vec![],
        }
    }
    pub fn get_folder(&self, parent: usize, folder_name: &str) -> Option<usize> {
        let children = self.get_children(parent);
        let mut children = children.iter();
        println!("{:?}", children);
        assert_eq!(self.nodes.len(), self.parents.len());
        loop {
            let idx = children.next();
            match idx {
                Some(idx) => {
                    match &self.nodes[*idx] {
                        Node::Folder { name, children } => {
                            if *name == folder_name {
                                return Some(*idx);
                            }
                        },
                        Node::File { name, size } => return None,
                    }
                },
                None => return None,
            }
        }
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

#[derive(Debug, Eq, PartialEq)]
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
    let mut current_folder = root;

    let mut token = TokenKind::None;
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut split = line.split(' ');
        let first_arg = split.next().unwrap();
        
        if first_arg.bytes().nth(0).unwrap() == b'$'
        {
            let command_type = split.next().unwrap();
            match command_type {
                "cd" => token = TokenKind::Cd(split.next().unwrap().to_owned()),
                "ls" => { token = TokenKind::Ls; continue },
                _ => ()
            }
        }

        // println!("tk {:#?}", token);

        match token {
            TokenKind::Ls => {
                if first_arg == "dir" {
                    let folder = Node::new_folder(split.next().unwrap());
                    tree.insert(folder, current_folder);
                }
                else {
                    let size = first_arg.parse::<i32>().unwrap();
                    let file = Node::new_file(split.next().unwrap(), size);
                    tree.insert(file, current_folder);
                }
            }
            TokenKind::Cd(ref path) => {
                if path == ".." {
                    current_folder = tree.get_parent(current_folder);
                    // println!("changing path .. {}", current_folder);
                }
                else if path == "/" {
                    // println!("resetting path /");
                    current_folder = 0;
                }
                else { // cd path
                    current_folder = tree.get_folder(current_folder, path.as_str()).unwrap();
                    // println!("changing path {}", current_folder);

                }
            },
            TokenKind::None => ()
        }


    }

    // println!("{:#?}", tree);
}
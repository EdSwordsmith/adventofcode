use std::{collections::HashMap, io};

#[derive(Debug)]
enum Node {
    File(usize),
    Dir(HashMap<String, Node>),
}

impl Node {
    fn size(&self) -> usize {
        match self {
            Node::File(file_size) => *file_size,
            Node::Dir(children) => children.values().map(|c| c.size()).sum(),
        }
    }

    fn dir_sizes(&self) -> Vec<usize> {
        if let Node::Dir(children) = self {
            let mut sizes: Vec<usize> = children
                .values()
                .filter(|c| matches!(c, Node::Dir(_)))
                .flat_map(|c| c.dir_sizes())
                .collect();
            sizes.push(self.size());

            return sizes;
        }

        panic!("What are you doing, bro?")
    }

    fn part_one(&self) -> usize {
        self.dir_sizes().iter().filter(|size| **size < 100000).sum()
    }

    fn part_two(&self) -> usize {
        let available = 70000000 - self.size();
        let need_to_delete = 30000000 - available;
        *self.dir_sizes().iter().filter(|size| **size > need_to_delete).min().unwrap()
    }
}

fn main() {
    let mut root = Node::Dir(HashMap::new());
    let mut dir_stack: Vec<*mut Node> = Vec::new();
    dir_stack.push(&mut root as *mut _);

    for line in io::stdin().lines() {
        let line = line.unwrap();

        if line.starts_with("$ cd") {
            let target = &line[5..];
            if target == ".." {
                dir_stack.pop();
            } else if target != "/" {
                unsafe {
                    if let Node::Dir(children) = dir_stack.last().unwrap().as_mut().unwrap() {
                        dir_stack.push(children.get_mut(target).unwrap() as *mut _);
                    }
                }
            }
        } else if !line.starts_with('$') {
            let (arg, name) = line.split_once(' ').unwrap();
            let node = if arg == "dir" {
                Node::Dir(HashMap::new())
            } else {
                Node::File(arg.parse().unwrap())
            };

            unsafe {
                if let Node::Dir(children) = dir_stack.last().unwrap().as_mut().unwrap() {
                    children.insert(name.to_owned(), node);
                }
            }
        }
    }

    println!("Part 1: {}", root.part_one());
    println!("Part 2: {}", root.part_two());
}

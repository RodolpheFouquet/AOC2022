use core::fmt;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
enum Command {
    ChangeDir(String),
    List,
}

impl TryFrom<String> for Command {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.split_whitespace();
        if split.next().is_none() {
            Err(())
        } else {
            match split.next() {
                Some("ls") => Ok(Self::List),
                Some("cd") => Ok(Self::ChangeDir(split.next().unwrap().to_string())),
                Some(_) => Err(()),
                None => Err(()),
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum NodeType {
    Directory(String),
    File((String, usize)),
}

impl TryFrom<String> for NodeType {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.split_whitespace();
        match split.next() {
            Some("dir") => Ok(Self::Directory(split.next().unwrap().to_string())),
            Some(c) => {
                let size = c.parse::<usize>().map_err(|_| ())?;
                let name = split.next().unwrap().to_string();
                Ok(Self::File((name, size)))
            }
            None => Err(()),
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Directory(name) => write!(f, "- {} (dir,", name),
            Self::File((name, _)) => write!(f, "- {} (file,", name),
        }
    }
}

struct Node {
    value: NodeType,
    level: u32,
    size: usize,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} size={})", self.value, self.size)?;
        self.children.iter().for_each(|c| {
            for _ in 0..c.borrow().level {
                write!(f, "  ").unwrap();
            }
            write!(f, "{}", c.borrow()).unwrap();
        });
        write!(f, "")
    }
}

impl Node {
    fn new_ref(node_type: NodeType) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value: node_type,
            level: 0,
            parent: None,
            children: Vec::new(),
            size: 0,
        }))
    }

    fn add_child(parent: Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
        let node_to_insert = Rc::clone(&child);
        if node_to_insert.borrow().parent.is_some() {
            return;
        }
        node_to_insert.borrow_mut().level = parent.borrow().level + 1;
        node_to_insert.borrow_mut().parent = Some(Rc::clone(&parent));
        parent.borrow_mut().children.push(node_to_insert.to_owned());
    }

    fn du(&mut self) -> usize {
        self.size = match self.value.clone() {
            NodeType::Directory(_) => self.children.iter().map(|c| c.borrow_mut().du()).sum(),
            NodeType::File((_, s)) => s,
        };
        self.size
    }

    // run du on root before!
    fn ex1(&self) -> usize {
        let contrib = match self.value.clone() {
            NodeType::Directory(_) if self.size < 100000 => self.size,
            _ => 0,
        };
        let children: usize = self.children.iter().map(|c| c.borrow_mut().ex1()).sum();
        contrib + children
    }

    // run du on root before!
    fn free(&self, free: usize, required: usize, sizes: &mut Vec<usize>)  {
        match self.value.clone() {
            NodeType::Directory(_) if free + self.size > required  => sizes.push(self.size),
            _ => (),
        };
        self.children.iter().for_each(|c| c.borrow().free(free, required, sizes));
    }
}

fn main() {
    let start = Instant::now();
    let content = fs::read_to_string("input.txt").expect("the file should be present");

    let root = Node::new_ref(NodeType::Directory(String::from("/")));
    root.borrow_mut().parent = Some(Rc::clone(&root));
    let mut current_node = Rc::clone(&root);
    let mut next_node = current_node.clone();

    content.lines().for_each(|line| {
        if line.starts_with('$') {
            let command = Command::try_from(line.to_string()).unwrap();
            if let Command::ChangeDir(dir_name) = command  {
                match dir_name.as_str() {
                    "/" => current_node = Rc::clone(&root),
                    ".." => {
                        let next = current_node.borrow().parent.clone().unwrap();
                        current_node = next;
                    }
                    s => {
                        let position =
                            current_node
                                .borrow()
                                .children
                                .clone()
                                .iter()
                                .position(|node| {
                                    matches!(node.borrow().value.clone(), NodeType::Directory(name) if name.as_str() == s)
                                });
                        match position {
                            Some(n) => {
                                next_node = current_node.borrow().children[n].clone();
                                current_node = Rc::clone(&next_node);
                            }
                            None => unreachable!(),
                        };
                    }
                }
            }

            
            // println!("{:?}", command)
        } else {
            let node_type = NodeType::try_from(line.to_string()).unwrap();
            Node::add_child(Rc::clone(&current_node), Node::new_ref(node_type.to_owned()));
        }
    });

    println!("running du on fs tree");
    root.borrow_mut().du();
    println!("{}", root.borrow());
    // println!("{}", root.borrow().ex1());
    let mut sizes: Vec<usize> = Vec::new();
    root.borrow().free(70000000 - root.borrow().size, 30000000, &mut sizes);
    //sizes.iter().for_each(|e| println!("{}", e));
    println!("the size to delete is {}", sizes.iter().min().unwrap());
    let duration = start.elapsed();
    println!("time spent {:?}", duration);
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn try_from_command() {
        let c = Command::try_from(String::from("$ cd /"));
        assert_eq!(c.unwrap(), Command::ChangeDir(String::from("/")));

        let c = Command::try_from(String::from("$ cd .."));
        assert_eq!(c.unwrap(), Command::ChangeDir(String::from("..")));

        let c = Command::try_from(String::from("$ ls"));
        assert_eq!(c.unwrap(), Command::List);
    }

    #[test]
    #[should_panic]
    fn try_from_command_fails() {
        Command::try_from(String::from("$ cd")).unwrap();
    }

    #[test]
    fn try_from_command_fails_2() {
        let c = Command::try_from(String::from("$"));
        assert!(c.is_err())
    }

    #[test]
    fn test_try_from_node_type() {
        let s = String::from("dir a");
        let t = NodeType::try_from(s);
        assert_eq!(t.unwrap(), NodeType::Directory(String::from("a")));

        let s = String::from("1290 ac");
        let t = NodeType::try_from(s);
        assert_eq!(t.unwrap(), NodeType::File((String::from("ac"), 1290)));

        let t = NodeType::try_from(String::from("a b"));
        assert!(t.is_err());
    }
}

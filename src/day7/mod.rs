use std::collections::HashMap;

use crate::*;

puzzle!("Day 7: No Space Left On Device", Solver, 95437, 24933642);

struct Solver {
    walker: Walker,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        let walker = input.split("$ ").filter(|cmd| !cmd.is_empty()).fold(
            Walker::default(),
            |mut walker, cmd| {
                let mut lines = cmd.lines();
                let cmd = lines.next().unwrap();
                let (cmd, arg) = cmd.split_once(' ').unwrap_or((cmd, ""));
                match cmd {
                    "ls" => {
                        let nodes = lines
                            .map(|data| {
                                let (meta, name) = data.split_once(' ').unwrap();
                                match meta {
                                    "dir" => Node::Dir(name.into()),
                                    size => Node::File(size.parse().unwrap()),
                                }
                            })
                            .collect();
                        walker.list(nodes)
                    }
                    "cd" => walker.cd(arg),
                    _ => unreachable!(),
                }
                walker
            },
        );
        Self { walker }
    }

    fn part_one(self) -> usize {
        self.walker
            .size_info
            .into_values()
            .filter(|s| *s <= 100_000)
            .sum()
    }

    fn part_two(self) -> usize {
        let free_space = 70_000_000 - self.walker.size_info.get("/").unwrap();
        let to_delete = 30_000_000 - free_space;
        let mut options = self
            .walker
            .size_info
            .into_values()
            .filter(|s| *s >= to_delete)
            .collect::<Vec<_>>();
        options.sort();
        options[0]
    }
}

#[derive(Debug)]
pub enum Node {
    Dir(String),
    File(usize),
}

#[derive(Debug, Default)]
pub struct Walker {
    pwd: Vec<String>,
    size_info: HashMap<String, usize>,
}

impl Walker {
    pub fn cd(&mut self, dir: &str) {
        match dir {
            "/" => {
                self.pwd.clear();
                self.pwd.push(dir.into());
            }
            ".." => {
                self.pwd.pop();
            }
            dir => {
                self.pwd.push(dir.into());
            }
        }
    }

    fn list(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            if let Node::File(s) = node {
                let mut pwd = self.pwd.clone();
                while !pwd.is_empty() {
                    let file_name = pwd.join("/");
                    *self.size_info.entry(file_name).or_insert(0) += s;
                    pwd.pop();
                }
            }
        }
    }
}

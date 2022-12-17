use std::borrow::ToOwned;

const DAY_NUM: &str = "7";

fn main() {
    let input = include_str!("../../inputs/input7.txt");
    let tree = create_tree(input);
    println!("Day{} Part1: {}", DAY_NUM, part1(&tree));
    println!("Day{} Part2: {}", DAY_NUM, part2(&tree));
}

fn part1(tree: &ArenaTree<FileItem>) -> usize {
    let all_branches = tree.all_child_branches(0);
    let mut proper_size = Vec::new();

    for branch in all_branches {
        let total: usize = tree
            .all_child_leafs(branch)
            .iter()
            .map(|&id| tree.get_node(id).unwrap().value.size)
            .sum();
        if total <= 100000 {
            proper_size.push(total);
        }
    }

    proper_size.iter().sum()
}

fn part2(tree: &ArenaTree<FileItem>) -> usize {
    let all_branches = tree.all_child_branches(0);
    let mut proper_size = Vec::new();

    let free_space_needed = 30000000;
    let disk_space = 70000000;

    let root_file_size = tree
        .all_child_leafs(0)
        .iter()
        .map(|x| tree.get_node(*x).unwrap().value.size)
        .sum::<usize>();
    for branch in all_branches {
        let total: usize = tree
            .all_child_leafs(branch)
            .iter()
            .map(|&id| tree.get_node(id).unwrap().value.size)
            .sum();
        if total <= free_space_needed {
            proper_size.push(total);
        }
    }

    let space_free = disk_space - root_file_size;
    let space_needed = free_space_needed - space_free;

    proper_size.sort_unstable();
    proper_size
        .iter()
        .find(|&x| x >= &space_needed)
        .map(ToOwned::to_owned)
        .unwrap()
}

#[derive(Debug, PartialEq)]
enum FileType {
    File,
    Directory,
}

#[derive(Debug, PartialEq)]
struct FileItem {
    name: String,
    size: usize,
    file_type: FileType,
}

impl FileItem {
    fn new_file(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
            file_type: FileType::File,
        }
    }
    fn new_dir(name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: 0,
            file_type: FileType::Directory,
        }
    }
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    id: usize,
    value: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(id: usize, value: T) -> Self {
        Self {
            id,
            value,
            parent: None,
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct ArenaTree<T>
where
    T: PartialEq + std::fmt::Debug,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq + std::fmt::Debug,
{
    fn new() -> Self {
        Self { arena: Vec::new() }
    }

    fn node(&mut self, value: T) -> usize {
        // Get node id if it already exists otherwise insert new node
        if let Some(node) = self.arena.iter().find(|&n| n.value == value) {
            node.id
        } else {
            let id = self.arena.len();
            self.arena.push(Node::new(id, value));
            id
        }
    }

    fn insert(&mut self, value: T) -> usize {
        let id = self.arena.len();
        self.arena.push(Node::new(id, value));
        id
    }

    fn parent_of(&self, node_id: usize) -> Option<usize> {
        if let Some(node) = self.arena.get(node_id) {
            node.parent
        } else {
            None
        }
    }

    fn find_child<F>(&self, parent: usize, predicate: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        self.arena[parent]
            .children
            .iter()
            .find(|&&x| predicate(&self.arena[x].value))
            .map(ToOwned::to_owned)
    }

    fn add_new_child(&mut self, parent: usize, value: T) -> Option<usize> {
        if parent >= self.arena.len() {
            return None;
        }

        let child = self.insert(value);
        self.arena[child].parent = Some(parent);
        self.arena[parent].children.push(child);

        Some(child)
    }

    fn get_node(&self, id: usize) -> Option<&Node<T>> {
        self.arena.get(id)
    }

    fn all_child_leafs(&self, from: usize) -> Vec<usize> {
        let mut leafs = Vec::new();

        // if we are a leaf return none
        if self.arena[from].children.is_empty() {
            return leafs;
        }

        // recurse through all child nodes and get all leaves
        for &child in &self.arena[from].children {
            if self.arena[child].children.is_empty() {
                leafs.push(child);
            } else {
                leafs.extend(self.all_child_leafs(child));
            }
        }

        leafs
    }

    fn all_child_branches(&self, from: usize) -> Vec<usize> {
        let mut branches = Vec::new();

        // if we are a leaf return none
        if self.arena[from].children.is_empty() {
            return branches;
        }

        // recurse through all child nodes and get all branches
        for &child in &self.arena[from].children {
            if !self.arena[child].children.is_empty() {
                branches.push(child);
                branches.extend(self.all_child_branches(child));
            }
        }

        branches
    }
}

fn create_tree(data: &str) -> ArenaTree<FileItem> {
    let mut tree: ArenaTree<FileItem> = ArenaTree::new();
    let mut current_node = tree.node(FileItem {
        name: "/".to_string(),
        size: 0,
        file_type: FileType::Directory,
    });

    for line in data.lines() {
        let tokens = line.split(' ').collect::<Vec<&str>>();
        if tokens[0] == "$" {
            match tokens[1] {
                "cd" => match tokens[2] {
                    ".." => {
                        if let Some(new_node) = tree.parent_of(current_node) {
                            current_node = new_node;
                        }
                    }
                    _ => {
                        if tokens[2] == "/" {
                            current_node = 0;
                        } else {
                            let Some(new_node) = tree
                                .find_child(current_node, |fi| {
                                    fi.name == tokens[2]
                                }) else {
                                    panic!("Failed to find node");
                                };
                            current_node = new_node;
                        }
                    }
                },
                "ls" => (),
                _ => {
                    panic!("Unsupported command");
                }
            }
        } else {
            let item = match tokens[0] {
                "dir" => FileItem::new_dir(tokens[1]),
                _ => FileItem::new_file(tokens[1], tokens[0].parse().unwrap()),
            };
            tree.add_new_child(current_node, item).unwrap();
        }
    }
    // dbg!(&tree);

    tree
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&create_tree(INPUT)), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&create_tree(INPUT)), 24933642);
    }
}

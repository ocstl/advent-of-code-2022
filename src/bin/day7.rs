use std::collections::hash_map::Iter;
use std::collections::HashMap;

const FILE: &str = "inputs/day7.txt";
const LIMIT: usize = 100000;
const TOTAL_DISK_SPACE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

type Directory = HashMap<String, Entry>;
type File = usize;

#[derive(Debug, Clone)]
pub enum Entry {
    Directory(Directory),
    File(File),
}

impl Entry {
    pub fn new_directory() -> Self {
        Entry::Directory(HashMap::new())
    }

    pub fn new_file(s: usize) -> Self {
        Entry::File(s)
    }

    pub fn is_directory(&self) -> bool {
        matches!(self, Entry::Directory(_))
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Entry::File(_))
    }

    pub fn add_entry(&mut self, name: &str, entry: Entry) {
        match self {
            Entry::Directory(h) => h.insert(name.to_string(), entry),
            Entry::File(_) => panic!("Attempting to add an entry to a file."),
        };
    }

    pub fn list(&self) -> Option<Iter<String, Entry>> {
        match self {
            Entry::Directory(h) => Some(h.iter()),
            Entry::File(_) => None,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Entry::Directory(h) => h.values().map(Entry::size).sum(),
            Entry::File(size) => *size,
        }
    }
}

fn read_input(input: &str) -> Entry {
    fn create_directory(lines: &mut std::str::Lines) -> Entry {
        let mut current = Entry::new_directory();

        'outer: while let Some(line) = lines.by_ref().next() {
            let mut words = line.split_whitespace();
            match words.next() {
                Some("$") => match words.next() {
                    Some("cd") => match words.next() {
                        Some("..") => break 'outer,
                        Some("/") => unimplemented!(),
                        Some(name) => {
                            let d = create_directory(lines.by_ref());
                            current.add_entry(name, d);
                        }
                        _ => unreachable!("Invalid cd argument."),
                    },
                    Some("ls") => {
                        while let Some(line) = lines.by_ref().next() {
                            let mut words = line.split_whitespace();
                            match words.next() {
                                Some("$") => match words.next() {
                                    Some("cd") => match words.next() {
                                        Some("..") => break 'outer,
                                        Some("/") => unimplemented!(),
                                        Some(name) => {
                                            let d = create_directory(lines.by_ref());
                                            current.add_entry(name, d);
                                        }
                                        _ => unreachable!("Invalid cd argument."),
                                    },
                                    _ => unreachable!(),
                                },
                                Some("dir") => current.add_entry(
                                    words.next().expect("Missing name."),
                                    Entry::new_directory(),
                                ),
                                Some(s) => current.add_entry(
                                    words.next().expect("Missing name."),
                                    Entry::new_file(s.parse().unwrap()),
                                ),
                                _ => unreachable!(),
                            }
                        }
                    }
                    _ => unreachable!("Invalid command."),
                },
                _ => unreachable!("Expecting a command."),
            }
        }

        current
    }

    let mut lines = input.lines();
    // This is always "$ cd /".
    let _ = lines.next();
    create_directory(lines.by_ref())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(FILE)?;
    let root = read_input(&input);

    // Find all of the directories with a total size of at most 100000.
    // What is the sum of the total sizes of those directories?
    let mut dir_sizes = Vec::new();
    let mut to_visit = vec![&root];
    while let Some(current) = to_visit.pop() {
        dir_sizes.push(current.size());

        if let Some(entries) = current.list() {
            to_visit
                .extend(entries.filter_map(|(_, v)| if v.is_directory() { Some(v) } else { None }));
        }
    }
    let part1: usize = dir_sizes.iter().filter(|&&s| s <= LIMIT).sum();
    println!("Part 1: {part1}");

    // Find the smallest directory that, if deleted, would free up enough space
    // on the filesystem to run the update. What is the total size of that
    // directory?
    let root_size = root.size();
    let part2 = dir_sizes
        .iter()
        .filter(|&&s| TOTAL_DISK_SPACE - root_size + s >= UPDATE_SIZE)
        .min()
        .expect("Deleting root would work.");
    println!("Part 2: {part2}");

    Ok(())
}

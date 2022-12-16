use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Directory {
    name: String,
    index: usize,
    parent: Option<usize>,
    directories: Vec<usize>,
    files: Vec<usize>,
}

#[derive(Debug)]
struct File {
    name: String,
    _parent: usize,
    size: usize,
}

const ROOT_INDEX: usize = 0usize;

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
    let (directories, files) = construct_filesytem(input.trim().lines());
    dbg!(&directories, &files);

    let mut dir_sizes = HashMap::<usize, usize>::new();

    let mut stack = VecDeque::new();
    stack.push_front(ROOT_INDEX);
    while let Some(dir_index) = stack.pop_front() {
        let dir = &directories[dir_index];
        if dir.directories.iter().any(|i| !dir_sizes.contains_key(i)) {
            stack.push_front(dir_index);
            for i in dir.directories.iter() {
                stack.push_front(*i);
            }
        } else {
            let dir_size: usize = dir.files.iter().map(|&i| files[i].size).sum::<usize>()
                + dir.directories.iter().map(|i| dir_sizes[i]).sum::<usize>();
            dir_sizes.insert(dir_index, dir_size);
        }
    }

    dbg!(&dir_sizes);

    let part_1_sum: usize = dir_sizes.values().filter(|&&it| it <= 100_000).sum();
    dbg!(part_1_sum);

    let available = 70_000_000;
    let used = available - dir_sizes[&ROOT_INDEX];
    let needed = 30_000_000 - used;
    let part_2_size: usize = *dir_sizes
        .values()
        .filter(|&&it| it >= needed)
        .min()
        .unwrap();
    dbg!(needed, part_2_size);
}

fn construct_filesytem(mut input_lines: std::str::Lines) -> (Vec<Directory>, Vec<File>) {
    let root_directory = Directory {
        name: "/".to_string(),
        index: ROOT_INDEX,
        parent: None,
        directories: Vec::new(),
        files: Vec::new(),
    };
    let mut directories: Vec<Directory> = vec![root_directory];
    let mut files: Vec<File> = Vec::new();
    let mut cwd_index = ROOT_INDEX;

    let mut next_line = input_lines.next();
    while let Some(cmd_line) = next_line {
        let cmd_prefix = "$ ";
        let cmd_line = cmd_line.trim_start_matches(cmd_prefix);

        let mut results = Vec::new();
        loop {
            next_line = input_lines.next();
            if let Some(line) = next_line {
                if line.starts_with(cmd_prefix) {
                    break;
                } else {
                    results.push(line);
                }
            } else {
                break;
            }
        }
        dbg!(cmd_line, &results);

        match cmd_line {
            "ls" => {
                for entry in results {
                    match entry.split_once(' ') {
                        Some(("dir", name)) => {
                            if directories[cwd_index]
                                .directories
                                .iter()
                                .all(|it| directories[*it].name != name)
                            {
                                let index = directories.len();
                                directories.push(Directory {
                                    name: name.to_string(),
                                    index,
                                    parent: Some(cwd_index),
                                    directories: Vec::new(),
                                    files: Vec::new(),
                                });
                                directories[cwd_index].directories.push(index);
                            }
                        }
                        Some((size, name)) => {
                            let cwd = &mut directories[cwd_index];
                            if cwd.files.iter().all(|&it| files[it].name != name) {
                                let index = files.len();
                                cwd.files.push(index);
                                files.push(File {
                                    name: name.to_string(),
                                    _parent: cwd.index,
                                    size: size.parse().unwrap(),
                                });
                            }
                        }
                        None => unreachable!(),
                    }
                }
            }
            _ => {
                let (cmd, arg) = cmd_line.split_once(' ').unwrap();
                match cmd {
                    "cd" => cwd_index = handle_cd(cmd, arg, &directories, cwd_index),
                    _ => unreachable!(),
                }
            }
        }
    }

    (directories, files)
}

fn handle_cd(cmd: &str, arg: &str, directories: &[Directory], cwd_index: usize) -> usize {
    dbg!(cmd, arg);
    match arg {
        "/" => ROOT_INDEX,
        ".." => match directories[cwd_index].parent {
            Some(parent) => parent,
            _ => cwd_index,
        },
        x => {
            directories[*directories[cwd_index]
                .directories
                .iter()
                .find(|&&it| directories[it].name == x)
                .unwrap()]
            .index
        }
    }
}

use std::{collections::HashMap, io::stdin, rc::Rc};

const SIZE_THRESHOLD: usize = 100000;

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> File {
        File {
            name: String::from(name),
            size,
        }
    }
}

#[derive(Clone, Debug)]
struct Dir {
    name: String,
    sub_dirs: HashMap<String, Dir>,
    files: HashMap<String, File>,
}

impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name,
            sub_dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn size(&self) -> usize {
        let mut ret = 0;
        for (_, v) in self.sub_dirs.iter() {
            ret += v.size();
        }
        for (_, v) in self.files.iter() {
            ret += v.size;
        }

        return ret;
    }

    fn with_subdir(&mut self, dir: String) -> &mut Dir {
        self.sub_dirs
            .entry(dir.clone())
            .or_insert_with(|| Dir::new(dir))
    }
}

fn main() {
    let mut root = Dir::new(String::from("/"));
    let mut trail: Vec<&mut Dir> = Vec::new();
    let mut cur_dir = &mut root;

    let lines: Vec<String> = stdin().lines().map(|x| x.unwrap()).collect();

    let mut iter = lines.iter();
    let mut line = iter.next();
    while line.is_some() {
        let cmd = line.unwrap().trim().strip_prefix("$").unwrap().trim();
        let cd_cmd = cmd.strip_prefix("cd ");
        if cd_cmd.is_some() {
            let dir_name = cd_cmd.unwrap().trim();
            cur_dir = match dir_name {
                ".." => match trail.pop() {
                    Some(d) => d,
                    None => cur_dir,
                },
                "/" => {
                    trail.clear();
                    &mut root
                }
                _ => {
                    trail.push(cur_dir);
                    cur_dir.with_subdir(dir_name.to_string())
                }
            }
        }

        line = iter.next();
    }
}

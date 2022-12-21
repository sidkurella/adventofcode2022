use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashMap, io::stdin};

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
    sub_dirs: HashMap<String, Rc<RefCell<Dir>>>,
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
            ret += v.borrow().size();
        }
        for (_, v) in self.files.iter() {
            ret += v.size;
        }

        return ret;
    }

    fn with_subdir(&mut self, dir: String) -> Rc<RefCell<Dir>> {
        let cell = self
            .sub_dirs
            .entry(dir.clone())
            .or_insert_with(|| Rc::new(RefCell::new(Dir::new(dir))));
        cell.clone()
    }

    fn add_file(&mut self, file: File) {
        self.files.insert(file.name.clone(), file);
    }
}

fn main() {
    let root = Rc::new(RefCell::new(Dir::new(String::from("/"))));
    let mut trail: Vec<Rc<RefCell<Dir>>> = Vec::new();
    let mut cur_dir = root.clone();

    let lines: Vec<String> = stdin().lines().map(|x| x.unwrap()).collect();

    let mut iter = lines.iter();
    let mut line = iter.next();
    while line.is_some() {
        let cmd = line.unwrap().trim().strip_prefix("$").unwrap().trim();
        if let Some(dir_name) = cmd.strip_prefix("cd ") {
            cur_dir = match dir_name {
                ".." => match trail.pop() {
                    Some(d) => d,
                    None => cur_dir,
                },
                "/" => {
                    trail.clear();
                    root.clone()
                }
                _ => {
                    trail.push(cur_dir.clone());
                    cur_dir.borrow_mut().with_subdir(dir_name.to_string())
                }
            }
        }

        line = iter.next();
    }
}

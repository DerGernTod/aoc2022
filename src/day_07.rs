use std::{fs, collections::HashMap, rc::Rc, cell::RefCell};

use self::file_tree::FileTree;

mod file_tree;
pub fn day_07() {
    let tree = read_into_file_tree("./input/day_07.txt");
    let sum = calc_dirs_lt_100k(&tree);
    println!("sum of dirs lt 100k {}", sum);
    println!("size of smallest eligible dir {}", find_smallest_delete_dir(&tree));
}

fn calc_dirs_lt_100k(files: &HashMap<String, Rc<RefCell<FileTree>>>) -> usize {
    files.values()
        .filter(|file| file.borrow().is_dir)
        .filter(|file| file.borrow().size <= 100000)
        .map(|file| file.borrow().size)
        .sum()
}

fn find_smallest_delete_dir(files: &HashMap<String, Rc<RefCell<FileTree>>>) -> usize {
    let root_size = files.get("/").unwrap().borrow().size;
    let required_size = 30_000_000 - (70_000_000 - root_size);
    let mut eligible_dirs: Vec<&Rc<RefCell<FileTree>>> = files.values()
        .filter(|file| file.borrow().is_dir)
        .filter(|file| file.borrow().size >= required_size)
        .collect();
    eligible_dirs.sort_by_key(|dir| dir.borrow().size);
    eligible_dirs.first().unwrap().borrow().size
}

fn command_into_file_tree<'a>((mut paths, mut map): (Vec<&'a str>, HashMap<String, Rc<RefCell<FileTree>>>), command: (&'a str, &'a str, Option<&'a str>)) -> (Vec<&'a str>, HashMap<String, Rc<RefCell<FileTree>>>) {
    let parent_path = paths.join("/");
    let parent_rc = map.get(&parent_path).map(Rc::clone);
    match command {
        ("$", "cd", Some("/")) => {
            paths.push("/");
            let full_path = paths.join("/");
            let tree = FileTree::new(0, parent_rc);
            map.insert(full_path, Rc::new(RefCell::new(tree)));
        },
        ("$", "cd", Some("..")) => {
            paths.pop();
        }
        ("$", "cd", Some(dir)) => {
            paths.push(dir);
        },
        ("dir", dirname, _) => {
            let full_path = paths.join("/") + "/" + dirname;
            let tree = FileTree::new(0, parent_rc);
            map.insert(full_path, Rc::new(RefCell::new(tree)));
        },
        (_, "ls", _) => (),
        (size, name, _) => {
            let full_path = paths.join("/") + "/" + name;
            let tree = FileTree::new(size.parse().unwrap(), parent_rc);
            map.insert(full_path, Rc::new(RefCell::new(tree)));
        }
    }
    (paths, map)
}

fn line_to_command(line: &str) -> (&str, &str, Option<&str>) {
    let mut words = line.split(' ');
    (words.next().unwrap(), words.next().unwrap(), words.next())
}

fn read_into_file_tree(path: &str) -> HashMap<String, Rc<RefCell<FileTree>>> {
    let input = fs::read_to_string(path).unwrap();
    let (_, files) = input
        .split('\n')
        .map(line_to_command)
        .fold((vec![], HashMap::new()), command_into_file_tree);
    files
}
#[cfg(test)]
mod tests {
    use super::{read_into_file_tree, calc_dirs_lt_100k, find_smallest_delete_dir};

    #[test]
    fn test_part_1() {
        let tree = read_into_file_tree("./input/day_07.test.txt");
        let sum = calc_dirs_lt_100k(&tree);
        assert_eq!(sum, 95437);
    }

    #[test]
    fn test_part_2() {
        let tree = read_into_file_tree("./input/day_07.test.txt");
        let smallest = find_smallest_delete_dir(&tree);
        assert_eq!(smallest, 24933642);
    }
}
use std::{fs, collections::HashMap, rc::Rc, cell::RefCell};

use self::file_tree::FileTree;

mod file_tree;
pub fn day_07() {
    let tree = read_into_file_tree("./input/day_07.txt");
    let sum = calc_dirs_lt_100k(&tree);
    println!("sum of dirs lt 100k {}", sum);
}

fn calc_dirs_lt_100k(files: &HashMap<String, Rc<RefCell<FileTree>>>) -> usize {
    files.values()
        .filter(|file| file.borrow().is_dir)
        .filter(|file| file.borrow().size <= 100000)
        .map(|file| file.borrow().size)
        .sum()
}

fn read_into_file_tree(path: &str) -> HashMap<String, Rc<RefCell<FileTree>>> {
    let input = fs::read_to_string(path).unwrap();
    let (_, files) = input
        .split('\n')
        .map(|line| {
            let mut words = line.split(' ');
            (words.next().unwrap(), words.next().unwrap(), words.next())
        })
        .fold((vec![], HashMap::new()), |(mut paths, mut map), command| {
            let parent_path = paths.join("/");
            let parent_rc = map.get(&parent_path).map(|parent| Rc::clone(&parent));
            match command {
                ("$", "cd", Some("/")) => {
                    paths.push("/");
                    let full_path = paths.join("/");
                    let tree = FileTree::new(String::from("/"), 0, parent_rc);
                    map.insert(full_path, Rc::new(RefCell::new(tree))).map(|_| panic!("Path already exists!"));
                },
                ("$", "cd", Some("..")) => {
                    paths.pop();
                }
                ("$", "cd", Some(dir)) => {
                    paths.push(dir);
                },
                ("dir", dirname, _) => {
                    let full_path = paths.join("/") + "/" + dirname;
                    let tree = FileTree::new(String::from(dirname), 0, parent_rc);
                    map.insert(full_path, Rc::new(RefCell::new(tree))).map(|_| panic!("Path already exists!"));
                },
                (_, "ls", _) => (),
                (size, name, _) => {
                    let full_path = paths.join("/") + "/" + name;
                    let tree = FileTree::new(String::from(name), size.parse().unwrap(), parent_rc);
                    map.insert(full_path, Rc::new(RefCell::new(tree))).map(|_| panic!("Path already exists!"));
                }
            }
            (paths, map)
        });
    files
}
#[cfg(test)]
mod tests {
    use super::{read_into_file_tree, calc_dirs_lt_100k};

    #[test]
    fn test_part_1() {
        let tree = read_into_file_tree("./input/day_07.test.txt");
        let sum = calc_dirs_lt_100k(&tree);
        assert_eq!(sum, 95437);
    }
}
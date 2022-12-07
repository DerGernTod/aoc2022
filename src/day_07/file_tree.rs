use std::{cell::RefCell, rc::{Rc, Weak}, borrow::BorrowMut};

struct FileTree {
    parent: Option<Rc<RefCell<FileTree>>>,
    name: String,
    size: usize,
    is_dir: bool
}

impl FileTree {
    pub fn new(name: String, size: usize, parent: Option<Rc<RefCell<FileTree>>>) -> FileTree {
        let new_parent = parent.map(|rc| {
            rc.borrow_mut().add_size(size);
            Rc::clone(&rc)
        });
        FileTree { parent: new_parent, name, size, is_dir: size == 0 }
    }
    pub fn add_size(&mut self, size: usize) {
        self.size += size;
        self.parent = self.parent.map(|rc| {
            let mut clone = Rc::clone(&rc.borrow());
            clone.add_size(size);
            RefCell::new(clone)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::FileTree;

    fn test_child_adding() {
        let mut root = FileTree::new(None, String::from("/"), 0);
        let mut child_file = FileTree::new(Some(&root), String::from("a"), 5);
    }
}
use std::{cell::{RefCell, RefMut}, rc::{Rc, Weak}, borrow::BorrowMut};

pub struct FileTree {
    parent: Option<Rc<RefCell<FileTree>>>,
    name: String,
    pub size: usize,
    pub is_dir: bool
}

impl FileTree {
    pub fn new(name: String, size: usize, parent: Option<Rc<RefCell<FileTree>>>) -> FileTree {
        if let Some(ref parent) = parent {
            parent.as_ref().borrow_mut().add_size(size);
        }
        FileTree { parent, name, size, is_dir: size == 0 }
    }
    fn add_size(&mut self, size: usize) {
        self.size += size;
        if let Some(ref parent) = self.parent {
            parent.as_ref().borrow_mut().add_size(size);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::RefCell};

    use super::FileTree;
    #[test]
    fn test_child_adding() {
        let root = FileTree::new(String::from("/"), 0, None);
        let root_rc = Rc::new(RefCell::new(root));
        FileTree::new(String::from("a"), 5, Some(Rc::clone(&root_rc)));
        FileTree::new(String::from("b"), 4, Some(Rc::clone(&root_rc)));
        let child_dir = FileTree::new(String::from("dir"), 0, Some(Rc::clone(&root_rc)));
        assert_eq!(root_rc.borrow().size, 9);
        let sub_rc = Rc::new(RefCell::new(child_dir));
        FileTree::new(String::from("child"), 3, Some(Rc::clone(&sub_rc)));
        assert_eq!(root_rc.borrow().size, 12);
        assert_eq!(sub_rc.borrow().size, 3);
    }
}
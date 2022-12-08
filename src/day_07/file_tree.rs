use std::{cell::{RefCell, RefMut}, rc::{Rc, Weak}, borrow::BorrowMut};

pub struct FileTree {
    parent: Option<Rc<RefCell<FileTree>>>,
    pub size: usize,
    pub is_dir: bool
}

impl FileTree {
    pub fn new(size: usize, parent: Option<Rc<RefCell<FileTree>>>) -> FileTree {
        if let Some(ref parent) = parent {
            parent.as_ref().borrow_mut().add_size(size);
        }
        FileTree { parent, size, is_dir: size == 0 }
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
        let root = FileTree::new(0, None);
        let root_rc = Rc::new(RefCell::new(root));
        FileTree::new(5, Some(Rc::clone(&root_rc)));
        FileTree::new(4, Some(Rc::clone(&root_rc)));
        let child_dir = FileTree::new(0, Some(Rc::clone(&root_rc)));
        assert_eq!(root_rc.borrow().size, 9);
        let sub_rc = Rc::new(RefCell::new(child_dir));
        FileTree::new(3, Some(Rc::clone(&sub_rc)));
        assert_eq!(root_rc.borrow().size, 12);
        assert_eq!(sub_rc.borrow().size, 3);
    }
}
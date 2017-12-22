use std::cmp::Ordering;

#[derive(PartialEq)]
pub struct BTree<T> {
    leaf: Option<T>,
    blow: Option<Box<BTree<T>>>,
    bhigh: Option<Box<BTree<T>>>,
}

impl<T: Ord> BTree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        BTree {
            leaf: None,
            blow: None,
            bhigh: None,
        }
    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
        match self.leaf {
            None => {
                self.leaf = Some(key);
                self.blow = Some(Box::new(BTree::new()));
                self.bhigh = Some(Box::new(BTree::new()));
                true
            },
            Some(ref val) => {
                match val.cmp(&key) {
                    Ordering::Equal => false,
                    Ordering::Less => self.bhigh.as_mut().unwrap().insert(key),
					Ordering::Greater => self.blow.as_mut().unwrap().insert(key),
                }
            },
        }
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.leaf {
            None => false,
            Some(ref val) => {
                match val.cmp(key) {
                    Ordering::Equal => true,
                    Ordering::Less => self.bhigh.as_ref().unwrap().find(key),
					Ordering::Greater => self.blow.as_ref().unwrap().find(key),
                }
            },
        }
    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        let mut res: Vec<&T> = Vec::new();
        match self.leaf {
            None => {
                res
            },
            Some(_) => {
                res.push(self.leaf.as_ref().unwrap());
                res.extend(self.blow.as_ref().unwrap().preorder());
                res.extend(self.bhigh.as_ref().unwrap().preorder());
                res
            }
        }
    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut res: Vec<&T> = Vec::new();
        match self.leaf {
            None => {
                res
            },
            Some(_) => {
                res = self.blow.as_ref().unwrap().inorder();
                res.push(self.leaf.as_ref().unwrap());
                res.extend(self.bhigh.as_ref().unwrap().inorder());
                res
            }
        }
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut res: Vec<&T> = Vec::new();
        match self.leaf {
            None => {
                res
            },
            Some(_) => {
                res.extend(self.blow.as_ref().unwrap().postorder());
                res.extend(self.bhigh.as_ref().unwrap().postorder());
                res.push(self.leaf.as_ref().unwrap());
                res
            }
        }
    }
}

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
            TreeNode {
                val,
                left: None,
                right: None
            }            
    }

    pub fn insert_recursively(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(n) = node {
            let mut n_borrow = n.borrow_mut();
            if val < n_borrow.val {
                if n_borrow.left.is_none() {
                    n_borrow.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                } else {
                    Self::insert_recursively(&n_borrow.left, val);
                }
            } else {
                if n_borrow.right.is_none() {
                    n_borrow.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                } else {
                    Self::insert_recursively(&n_borrow.right, val);
                }
            }
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut results: Vec<i32> = Vec::new();
        let mut q = VecDeque::new();
        q.push_back(root.clone());

        let mut height = 0;

        while !q.is_empty() {
            height += 1;
            for i in 0..q.len() {
                if let Some(node) = q.pop_front() {
                    if let Some(n) = node {
                        let n_ref = n.borrow();
                        results.push(n_ref.val);
                        if let Some(left) = &n_ref.left {
                            q.push_back(Some(left.clone()));
                        }
                        if let Some(right) = &n_ref.right {
                            q.push_back(Some(right.clone()));
                        }
                    }
                }
            }
        }

        println!("results={:?}", results);
        println!("height={}", height);
        height
    }

}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    TreeNode::insert_recursively(&root, 1);
    TreeNode::insert_recursively(&root, 20);
    TreeNode::insert_recursively(&root, 15);
    TreeNode::insert_recursively(&root, 23);
    
    TreeNode::max_depth(root);
}

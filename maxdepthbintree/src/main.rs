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

    pub fn max_depth_optimizied(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left_depth = Self::max_depth_optimizied(node.left.clone());
            let right_depth = Self::max_depth_optimizied(node.right.clone());
            return 1 + left_depth.max(right_depth);
        }
        0
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut results: Vec<i32> = vec![];
        let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        q.push_back(root);
        let mut height = 0;

        while !q.is_empty() {
            height += 1;
            for i in 0..q.len() {
                if let Some(Some(node)) = q.pop_front() {
                    let n = node.borrow_mut();
                    results.push(n.val);
                    if let Some(left) = &n.left {
                        q.push_back(Some(left.clone()));
                    }
                    if let Some(right) = &n.right {
                        q.push_back(Some(right.clone()));
                    }
                }            
            }
        }

        height
    }

}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    TreeNode::insert_recursively(&root, 1);
    TreeNode::insert_recursively(&root, 20);
    TreeNode::insert_recursively(&root, 15);
    TreeNode::insert_recursively(&root, 23);
    
    println!("{}", TreeNode::max_depth_optimizied(root));
}

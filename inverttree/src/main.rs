use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None
        }
    }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) {
        let mut results: Vec<i32> = vec![];
        let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let pointer = &root.clone();
        q.push_back(root);

        while !q.is_empty() {
            for i in 0..q.len() {
                if let Some(Some(node)) = q.pop_front() {
                    let mut node = node.borrow_mut();
                    if let (Some(left), Some(right)) =
                        (node.left.as_ref(), node.right.as_ref())
                    {
                        let tmp = node.left.clone();
                        node.left = node.right.clone();
                        node.right = tmp;
                        q.push_back(node.left.clone());
                        q.push_back(node.right.clone());
                    }
                }
            }
        }

        let p = pointer;
        println!("
            {:?}", pointer);
        Self::bfs(pointer);
    }

    pub fn bfs(root: &Option<Rc<RefCell<TreeNode>>>) {
        let mut results: Vec<i32> = vec![];
        let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        q.push_back(root.clone());

        while !q.is_empty() {
            for i in 0..q.len() {
                if let Some(Some(node)) = q.pop_front() {
                    let n = node.borrow();
                    results.push(n.val);

                    if let Some(left) = n.left.as_ref() {
                        let my = Some(left.clone());
                        q.push_back(my); 
                    }
                    if let Some(right) = n.right.as_ref() {
                        let my = Some(right.clone());
                        q.push_back(my); 
                    }
                }
            }
        }
        println!("res after bfs:{:?}", results);
    }

    pub fn insert(current_node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(node) = current_node {
            let mut n_borrow_mut = node.borrow_mut();
            if val < n_borrow_mut.val {
                if let Some(left) = n_borrow_mut.left.as_ref() {
                    Self::insert(&Some(left.clone()), val);
                } else {
                    n_borrow_mut.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                }
            } else {
                if let Some(right) = n_borrow_mut.right.as_ref() {
                    Self::insert(&Some(right.clone()), val);
                } else {
                    n_borrow_mut.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                }
            }
        }
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    TreeNode::insert(&root, 2);
    TreeNode::insert(&root, 7);
    TreeNode::insert(&root, 1);
    TreeNode::insert(&root, 3);
    TreeNode::insert(&root, 6);
    TreeNode::insert(&root, 9);

    TreeNode::invert_tree(root);
}

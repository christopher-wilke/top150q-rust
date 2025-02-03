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

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(n) => {
                let mut node = n.borrow_mut();

                // We only compare when we are at the bottom
                if node.left.is_none() && node.right.is_none() {
                    return target_sum - node.val == 0;
                }

                return Self::has_path_sum(node.left.take(), target_sum - node.val) ||
                Self::has_path_sum(node.right.take(), target_sum - node.val);
            },
            None => return false
        }
    }
}

fn main() {
    let mut tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                })))
            }))),
            right: None
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                })))
            })))
        })))
    })));

    let result = TreeNode::has_path_sum(tree, 22);
    println!("{:?}", result);
}

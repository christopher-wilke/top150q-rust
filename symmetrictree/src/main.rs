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

    pub fn check_arm(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>
    ) -> bool {
        match (left, right) {
            (Some(left), Some(right)) => {
                let mut left = left.borrow_mut();
                let mut right = right.borrow_mut();

                left.val == right.val
                    && Self::check_arm(left.left.take(), right.right.take())
                    && Self::check_arm(left.right.take(), right.left.take())
            }
            (None, None) => true,
            _ => false
        }
    }
    
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            return Self::check_arm(node.left.take(), node.right.take());
        } else {
         return false;   
        }
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            })))
        }))),
        right:Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            })))
        }))) 
    })));

    let result = TreeNode::is_symmetric(root);
    println!("res={:?}", result);
}

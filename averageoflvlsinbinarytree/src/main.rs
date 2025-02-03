use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

    pub fn insert(current_node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
        match current_node {
            Some(node) => {
                let mut node = node.borrow_mut();
                if val < node.val {
                    match &node.left {
                        Some(left) => {
                            Self::insert(&Some(left.clone()), val);
                        },
                        None => node.left = Some(Rc::new(RefCell::new(TreeNode::new(val))))
                    }
                } else {
                      match &node.right {
                        Some(right) => {
                            Self::insert(&Some(right.clone()), val);
                        },
                        None => node.right = Some(Rc::new(RefCell::new(TreeNode::new(val))))
                    }
                }
            },
            _ => ()
        }
    }

    pub fn get_lvl_avg(q: &VecDeque<Option<Rc<RefCell<TreeNode>>>>) -> f64 {
        let mut sum = 0.;
        for node in q {
            let node = node.as_ref().unwrap().borrow();
            sum += node.val as f64;
        }
        sum / q.len() as f64
    }

    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut values: Vec<f64> = Vec::new();
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);
        let mut resp: Vec<f64> = Vec::new();
        
        while !queue.is_empty() {
            let v = Self::get_lvl_avg(&queue);
            resp.push(v);
            for _ in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    let node = node.borrow_mut();
                    values.push(node.val as f64);

                    if let Some(left) = &node.left {
                        queue.push_back(Some(left.clone()));
                    }
                    if let Some(right) = &node.right {
                        queue.push_back(Some(right.clone()));
                    }
                }                
            }
        }
        resp        
    }

}

fn main() {
    // let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    // TreeNode::insert(&root, 9);
    // TreeNode::insert(&root, 20);
    // TreeNode::insert(&root, 15);
    // TreeNode::insert(&root, 7);
    
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None
            })))
        })))
    })));

    let values = TreeNode::average_of_levels(root);
    println!("{:?}", values);    
}

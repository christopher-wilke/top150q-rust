use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node 
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None
        }
    }

    pub fn insert(current_node: &Node, val: i32) {
         match current_node {
             Some(node) => {
                 let mut node = node.borrow_mut();
                 if val < node.val {
                     match node.left.as_ref() {
                         Some(left) => {
                             Self::insert(&Some(left.clone()), val)
                         },
                         None => node.left = Some(Rc::new(RefCell::new(TreeNode::new(val))))
                     }
                 } else {
                     match node.right.as_ref() {
                         Some(right) => {
                             Self::insert(&Some(right.clone()), val)
                         },
                         None => node.right = Some(Rc::new(RefCell::new(TreeNode::new(val))))
                     }                         
                 }
             },
             None => ()
        }
    }

    pub fn dfs(current: &Node, result: &mut Vec<i32>) {
        if let Some(node) = current {
            let node_b = node.as_ref().borrow();
            Self::dfs(&node_b.left, result);
            let v = node_b.val;
            result.push(v);
            Self::dfs(&node_b.right, result);
        }
    }

    pub fn calculate_min(res: &Vec<i32>) -> i32 {
        let mut current_dif = res[1] - res[0];

        for i in 1..res.len() - 1 {
            let dif = res[i+1] - res[i];
            if dif < current_dif {
                current_dif = dif;
            }
        }
    
        current_dif
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut results: Vec<i32> = vec![];
        Self::dfs(&root, &mut results);
        Self::calculate_min(&results)
    }
}

fn main() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    TreeNode::insert(&root, 2);
    TreeNode::insert(&root, 6);
    TreeNode::insert(&root, 1);
    TreeNode::insert(&root, 3);

    let dif  =TreeNode::get_minimum_difference(root);
    println!("dif={}", dif);
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

pub struct Solution;

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None            
        }
    }
}

impl Solution {

    pub fn insert(current_node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(node) = current_node {
            let mut node = node.borrow_mut();
            if val < node.val {
                match node.left.as_ref() {
                    Some(left) => {
                       Self::insert(&Some(left.clone()), val); 
                    },
                    None => node.left = Some(Rc::new(RefCell::new(TreeNode::new(val))))
                }
            }
            else {
                match node.right.as_ref() {
                    Some(right) => {
                       Self::insert(&Some(right.clone()), val); 
                    },
                    None => node.right = Some(Rc::new(RefCell::new(TreeNode::new(val))))
                }                
            }
        }
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {

        let sorted = Solution::balanced_order(nums);
        println!("sorted={:?}", sorted);
        let root = Some(Rc::new(RefCell::new(TreeNode::new(sorted[0]))));

        for i in 1..sorted.len() {
            Solution::insert(&root, sorted[i]);
        }
        
        root
    }

    pub fn balanced_order(arr: Vec<i32>) -> Vec<i32> {
        if arr.is_empty() {
            return vec![];
        }

        let mid = arr.len() / 2;
        let mut result = vec![arr[mid]];
        println!("{:?}", result);

        result.extend(Self::balanced_order(arr[..mid].to_vec()));
        result.extend(Self::balanced_order(arr[mid+1..].to_vec()));
        
        result
    }
}

fn main() {
    let arr = [0,1,2,3,4,5].to_vec();
    println!("{:?}", arr);
    let balanced = Solution::sorted_array_to_bst(arr);
}

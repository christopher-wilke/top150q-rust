use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use std::str::EscapeUnicode;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}

pub type Node = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None
        }
    }

    pub fn tree_cmp(p: &Node, q: &Node) {
        match (p, q) {
            (Some(a), Some(b)) => {
                let a_borrow = a.borrow();
                let b_borrow = b.borrow();
                if a_borrow.val != b_borrow.val {
                    println!("Error occured");
                } else {
                    if let (
                        Some(l1), Some(l2)) =
                        (a_borrow.left.as_ref(), b_borrow.left.as_ref()
                    )
                    {
                        Self::tree_cmp(&Some(l1.clone()), &Some(l2.clone()));
                    }
                }
            }
           _ => {} 
        }
    }

    pub fn single_dfs(p: &Node) {
        match p {
            Some(node) => {
                let v = node.borrow();
                if let Some(left) = v.left.as_ref() {
                    Self::single_dfs(&Some(left.clone()));                    
                }
                println!("v={}", v.val);
                if let Some(right) = v.right.as_ref() {
                    Self::single_dfs(&Some(right.clone()));                    
                }
            },
            None => {}
        }
    }

    pub fn dfs(p: &Node, q: &Node) -> bool {
        match (
                p.as_ref().map(|x| x.borrow()),
                q.as_ref().map(|y| y.borrow())
            ) {
                (Some(a), Some(b)) => {
                    a.val == b.val &&
                    Self::dfs(&a.left, &b.left) &&
                    Self::dfs(&a.right, &b.right)
                },
                (None, None) => true,
                _ => false
        }
    }

    pub fn bfs(p: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        if p.is_none() {
            return vec![];
        }

        let mut values: Vec<Option<i32>> = Vec::new();
        let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        q.push_back(p);

        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(Some(node)) = q.pop_front() {
                    let node_ref = node.borrow();
                    values.push(Some(node_ref.val));

                    if let Some(left) = &node_ref.left {
                        q.push_back(Some(left.clone()));
                    } else {
                        values.push(None);
                    }
                    if let Some(right) = &node_ref.right {
                        q.push_back(Some(right.clone()));
                    } else {
                        values.push(None);
                    }
                }
            }
        }
        values
    }
}

fn main() {
    let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    tree1.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    tree1.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    TreeNode::single_dfs(&tree1);

    // let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // tree2.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    // let res = TreeNode::dfs(&tree1, &tree2);
    // println!("{:?}", res);
}




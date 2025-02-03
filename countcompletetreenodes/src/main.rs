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

    pub fn traverse_dfs(
            left: Option<Rc<RefCell<TreeNode>>>,
            right: Option<Rc<RefCell<TreeNode>>>,
            results: &mut Vec<i32> 
        )
        {
        match (left, right) {
            (Some(l), Some(r)) => {
                let mut l_b = l.borrow_mut();
                let mut r_b = r.borrow_mut();

                println!("{} - {}", l_b.val, r_b.val);
                results.push(l_b.val);
                results.push(r_b.val);

                Self::traverse_dfs(l_b.left.take(), l_b.right.take(), results);
                Self::traverse_dfs(r_b.left.take(), r_b.right.take(), results);
            },
            (Some(x), None) => {
                let mut node = x.borrow_mut();
                println!("{}", node.val);
                results.push(node.val);
                Self::traverse_dfs(node.left.take(), node.right.take(),results)
            },
            (None, Some(x)) => {
                let mut node = x.borrow_mut();
                println!("{}", node.val);
                results.push(node.val);
                Self::traverse_dfs(node.left.take(), node.right.take(), results)
            }
            _ => {}
        }
    }

    pub fn traverse_bfs(current_node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
        let mut result: Vec<i32> = Vec::new();
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(current_node);

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    let mut node = node.borrow_mut();
                    result.push(node.val);

                    if let Some(left) = node.left.take() {
                        queue.push_back(Some(left));
                    }
                    if let Some(right) = node.right.take() {
                        queue.push_back(Some(right));
                    }
                }
            }
        }
        result.len() as i32
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Self::traverse(root)
        1
    }
}

fn main() {
    let mut tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None
            })))
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None
            }))),
            right: None
        })))
    })));

    let t = tree.as_ref();

    let left = t.unwrap().borrow().left.clone();
    let right = t.unwrap().borrow().right.clone();

    let mut results: Vec<i32> = vec![];
    TreeNode::traverse_dfs(left, right, &mut results);
    println!("results={:?}", results.len() + 1);
    // let result = TreeNode::count_nodes(tree);
    // println!("len = {}", result);
}

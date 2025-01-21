#[derive(Debug)]
pub struct Tree {
    root: Option<Box<Node>>
}

#[derive(Debug)]
pub struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Tree {
    pub fn new(val: i32) -> Self {
        Self { root: Node::new(val) }
    }

    pub fn insert_iterative(val: i32, root: &mut Box<Node>) {
        let mut current = root;

        loop {
            if val < current.value {
                let left = &mut current.left;
                match left {
                    Some(node) => {
                        current = node;
                    },
                    None => {
                        *left = Node::new(val);
                        break;
                    }
                } 
            } else {
                let right = &mut current.right;
                match right {
                    Some(node) => {
                        current = node;
                    },
                    None => {
                        *right = Node::new(val);
                        break;
                    }
                } 
            }
        }
    }

    pub fn insert_recursive(val: i32, current: &mut Box<Node>) {
        if val < current.value {
            match &mut current.left {
                Some(node) => Self::insert_recursive(val, node),
                None => current.left = Node::new(val)
            }
        } else {
            match &mut current.right {
                Some(node) => Self::insert_recursive(val, node),
                None => current.right = Node::new(val)
            }
        }
    }

    

    pub fn traverse_inorder_iteratively(root: Option<&Box<Node>>, results: &mut Vec<i32>) -> Vec<i32> {
        let mut queue: Vec<&Box<Node>> = Vec::new();
        let mut current = root;

        while queue.len() > 0 || current.is_some() {
            while let Some(node) = current {
                queue.push(node);
                current = node.left.as_ref();
            }
            if let Some(n) = queue.pop() {
                results.push(n.value);
                current = n.right.as_ref();
            }
        }
        results.to_vec()
    }

    pub fn traverse_recursively(current: &Box<Node>, results: &mut Vec<i32>) -> Vec<i32> {
        if let Some(left) = current.left.as_ref() {
            Self::traverse_recursively(left, results);
        }
        results.push(current.value); 
        if let Some(right) = current.right.as_ref() {
            Self::traverse_recursively(right, results);
        }
        results.to_vec()
    }

    pub fn traverse(&self) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();
        Self::traverse_inorder_iteratively(self.root.as_ref(), &mut results);
        results
    }

    pub fn insert(&mut self, val: i32) {
        if self.root.is_none() {
            self.root = Node::new(val);
        } else {
            let root = self.root.as_mut().unwrap();
            // Self::insert_recursive(val, root);
            Self::insert_iterative(val, root);
        }
    }
}

impl Node {
    pub fn new(value: i32) -> Option<Box<Self>> {
        Some(
            Box::new(Node { value, left: None, right: None })
        )
    }
}

pub fn main() {}

mod tests {
    use super::*;

    pub fn create_tree() -> Tree {
        let mut tree = Tree::new(8);
        tree.insert(3);
        tree.insert(10);
        tree.insert(1);
        tree.insert(6);
        tree.insert(14);
        tree.insert(4);
        tree.insert(7);
        tree.insert(13);
        tree 
    }
    
    #[test]
    pub fn should_traverse_inorder() {
        let tree = tests::create_tree();
        tree.traverse();
    }
    
    #[test]
    pub fn should_create_tree() {

        let tree = tests::create_tree();

        let third_left_val = tree
            .root
            .unwrap()
            .left
            .unwrap()
            .left
            .unwrap()
            .value;

        assert_eq!(third_left_val, 1);
    }
}

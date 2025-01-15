#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None
        }
    }
}

impl Tree {
    fn new() -> Self {
        Self {
            root: None
        }
    }

    fn insert(&mut self, value: i32) {
        // match &mut self.root {
        //     None => self.root = Node::new(value).into(),
        //     Some(node) => Self::insert_recursive(node, value)
        // }
        self.insert_iterative(value); 
    }

    fn insert_iterative(&mut self, val: i32) {
        if self.root.is_none() {
            self.root = Node::new(val).into();
            return;
        }

        let mut q: Vec<&mut Box<Node>> = Vec::new();
        let root = self.root.as_mut().unwrap();
        q.push(root);

        while let Some(node) = q.pop() {
            if val > node.value {
                let right = &mut node.right;
                match right {
                    Some(node) => {
                        q.push(node);
                    },
                    None => {
                        *right = Node::new(val).into()
                    }
                }
            }
            else {
                let left = &mut node.left;
                match left {
                    Some(node) => {
                        q.push(node);
                    },
                    None => {
                        *left = Node::new(val).into()
                    }
                }
            }
        } 
    }

    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value > node.value {
            match &mut node.right {
                None => node.right = Node::new(value).into(),
                Some(v) => Tree::insert_recursive(v, value)
            }
        } else {
            match &mut node.left {
                None => node.left = Node::new(value).into(),
                Some(v) => Tree::insert_recursive(v, value)
            }
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn works_builds_tree() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);
    
        assert_eq!(tree.root.is_some(), true);
        println!("{:?}", tree);
    }

}
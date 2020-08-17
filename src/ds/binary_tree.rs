use std::ops::Deref;

#[derive(Debug)]
pub enum BinaryTreeNode<T> {
    Node {
        value: T,
        children: (Box<BinaryTreeNode<T>>, Box<BinaryTreeNode<T>>),
    },
    Leaf,
}

impl<T> BinaryTreeNode<T> {
    pub fn new(value: T) -> Self {
        Self::Node {
            value,
            children: (Box::new(Self::Leaf), Box::new(Self::Leaf)),
        }
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            BinaryTreeNode::Node { value: v, children: (_, _) } => { Some(v) }
            BinaryTreeNode::Leaf => None,
        }
    }

    pub fn left(&self) -> &BinaryTreeNode<T> {
        match self {
            BinaryTreeNode::Node { value: _, children: (l, _) } => { l.deref() }
            BinaryTreeNode::Leaf => self,
        }
    }

    pub fn right(&self) -> &BinaryTreeNode<T> {
        match self {
            BinaryTreeNode::Node { value: _, children: (_, r) } => { r.deref() }
            BinaryTreeNode::Leaf => self,
        }
    }
}

pub struct DfsIter<'r, T> {
    // a stack which contains current node and a state
    stack: Vec<(&'r BinaryTreeNode<T>, u8)>
}

impl<'r, T> Iterator for DfsIter<'r, T> {
    type Item = &'r T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some((BinaryTreeNode::Leaf, _)) => self.next(),
            Some((node, state)) => {
                match state {
                    0 => {
                        self.stack.push((node, 1));
                        Some(node.value().unwrap())
                    }

                    1 => {
                        self.stack.push((node, 2));
                        self.stack.push((node.left(), 0));

                        self.next()
                    }

                    2 => {
                        self.stack.push((node, 3));
                        self.stack.push((node.right(), 0));

                        self.next()
                    }

                    _ => self.next()
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    pub root: BinaryTreeNode<T>
}

impl<T> BinaryTree<T> {
    pub fn new(root: T) -> Self {
        Self {
            root: BinaryTreeNode::new(root)
        }
    }

    pub fn dfs(&self) -> DfsIter<T> {
        DfsIter {
            stack: vec![(&self.root, 0)]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ds::binary_tree::{BinaryTree, BinaryTreeNode};

    #[test]
    fn dfs() {
        let mut root = BinaryTree {
            root: BinaryTreeNode::Node {
                value: 0,
                children: (
                    BinaryTreeNode::Node {
                        value: 1,
                        children: (
                            BinaryTreeNode::new(3).into(),
                            BinaryTreeNode::new(4).into(),
                        ),
                    }.into(),
                    BinaryTreeNode::new(2).into()
                ),
            }
        };

        println!("{:?}", root);

        for node in root.dfs() {
            println!("{:?}", node);
        }
    }
}
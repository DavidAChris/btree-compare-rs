#![allow(unused)]

struct TNode {
    value: i32,
    left: Option<Box<TNode>>,
    right: Option<Box<TNode>>,
}

impl TNode {
    pub fn new(
        value: i32,
        left: Option<Box<TNode>>,
        right: Option<Box<TNode>>,
    ) -> Option<Box<TNode>> {
        Some(Box::new(TNode { value, left, right }))
    }
}

fn compare(a: Option<Box<TNode>>, b: Option<Box<TNode>>) -> bool {
    if a.is_none() && b.is_none() {
        return true;
    }

    if a.is_none() || b.is_none() {
        return false;
    }
    let a = a.unwrap();
    let b = b.unwrap();
    if a.value != b.value {
        return false;
    }
    compare(a.left, b.left) && compare(a.right, b.right)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tnode_comparison() {
        let my_node = TNode::new(21, TNode::new(13, None, None), TNode::new(13, None, None));
        let your_node = TNode::new(21, TNode::new(13, None, None), TNode::new(13, None, None));
        assert_eq!(compare(my_node, your_node), true);
    }
}

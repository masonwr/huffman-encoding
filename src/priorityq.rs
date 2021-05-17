use crate::encoding::EncodingNode;

#[derive(Debug)]
pub struct PriorityQ {
    node: EncodingNode,
    next: Option<Box<PriorityQ>>,
}

impl PriorityQ {
    pub fn new(node: EncodingNode) -> Self {
        PriorityQ { node, next: None }
    }

    fn push(self, node: EncodingNode) -> Self {
        let next = Some(Box::new(self));
        PriorityQ { node, next }
    }

    pub fn pop(self) -> (EncodingNode, Option<Box<Self>>) {
        (self.node, self.next)
    }

    pub fn len(&self) -> usize {
        match &self.next {
            None => 1,
            Some(tail) => 1 + tail.len(),
        }
    }

    pub fn enque(self, node: EncodingNode) -> Self {
        if self.node.cmp(&node) == std::cmp::Ordering::Greater {
            return self.push(node);
        }

        let (val, tail) = self.pop();

        match tail {
            Some(tail) => tail.enque(node).push(val),
            None => PriorityQ::new(node).push(val),
        }
    }

    // reduce priority que to huffman encoding tree
    pub fn reduce(self) -> EncodingNode {
        let mut queue = self;

        // pop two nodes off, combine them into one, until there is only
        // one left, the root of the encoding tree
        let root = loop {
            let (n1, tail) = match queue.pop() {
                (n, Some(t)) => (n, t),
                (n, None) => break n,
            };

            let (n2, tail) = match tail.pop() {
                (n, Some(t)) => (n, t),
                (n2, None) => break EncodingNode::join(n1, n2),
            };

            let new_node = EncodingNode::join(n1, n2);
            queue = tail.enque(new_node);
        };

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn join_leaves() -> anyhow::Result<()> {
        let queue = PriorityQ::new(EncodingNode::new_leaf(0xff, 20));
        let queue = queue.enque(EncodingNode::new_leaf(0xff, 10));
        let queue = queue.enque(EncodingNode::new_leaf(0xff, 30));

        let (n, queue) = queue.pop();
        assert_eq!(n.count(), &10);

        let (n, queue) = queue.unwrap().pop();
        assert_eq!(n.count(), &20);

        let (n, queue) = queue.unwrap().pop();
        assert_eq!(n.count(), &30);

        assert_eq!(true, queue.is_none());

        Ok(())
    }
}

use crate::priorityq::PriorityQ;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;

use std::rc::Rc;

#[derive(Debug)]
pub enum EncodingNode {
    Leaf {
        byte: u8,
        count: usize,
    },
    Node {
        count: usize,
        left: Box<EncodingNode>,
        right: Box<EncodingNode>,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

pub type SymbolTable = HashMap<u8, Vec<Direction>>;

pub fn huffman_tree(hist: &BTreeMap<u8, usize>) -> anyhow::Result<EncodingNode> {
    Ok(PriorityQ::from(&hist)?.reduce())
}

impl EncodingNode {
    pub fn join(n1: EncodingNode, n2: EncodingNode) -> Self {
        let count = n1.count() + n2.count();
        let (left, right) = match n1.cmp(&n2) {
            std::cmp::Ordering::Less => (n1, n2),
            _ => (n2, n1),
        };

        Self::Node {
            count,
            left: left.into(),
            right: right.into(),
        }
    }

    pub fn count(&self) -> &usize {
        match self {
            Self::Leaf { count, .. } => count,
            Self::Node { count, .. } => count,
        }
    }

    fn byte(&self) -> Option<u8> {
        match self {
            Self::Leaf { byte, .. } => Some(*byte),
            _ => None,
        }
    }

    pub fn new_leaf(byte: u8, count: usize) -> Self {
        Self::Leaf { byte, count }
    }

    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        // todo revisit this, not sure if this is right...
        match self.count().cmp(other.count()) {
            // if eq, check byte
            Equal => {
                let sb = self.get_min_node().byte().unwrap_or_default();
                let ob = other.get_min_node().byte().unwrap_or_default();
                sb.cmp(&ob)
            }
            order => order,
        }
    }

    pub fn to_symbol_table(&self) -> SymbolTable {
        let st = Rc::new(RefCell::new(HashMap::new()));
        impl_make_st(st.clone(), self, vec![]);
        st.take()
    }

    fn get_min_node(&self) -> &Self {
        match self {
            Self::Leaf { .. } => self,
            Self::Node { left, .. } => left.get_min_node(),
        }
    }
}

fn impl_make_st(table: Rc<RefCell<SymbolTable>>, tree: &EncodingNode, path: Vec<Direction>) {
    match tree {
        EncodingNode::Leaf { byte, .. } => {
            table.borrow_mut().insert(*byte, path);
        }
        EncodingNode::Node { left, right, .. } => {
            impl_make_st(table.clone(), *&left, build_path(&path, Direction::Left));
            impl_make_st(table.clone(), *&right, build_path(&path, Direction::Right));
        }
    };
}

fn build_path(path: &[Direction], next: Direction) -> Vec<Direction> {
    let mut p = path[..].to_vec();
    p.push(next);
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn join_leaves() -> anyhow::Result<()> {
        let l1 = EncodingNode::new_leaf(0xff, 3);
        let l2 = EncodingNode::new_leaf(0xee, 7);

        let node = EncodingNode::join(l1, l2);

        match &node {
            EncodingNode::Node {
                count: 10,
                left: ln,
                right: rn,
            } => {
                assert_eq!(ln.count(), &3);
                assert_eq!(ln.byte(), Some(0xff));

                assert_eq!(rn.count(), &7);
                assert_eq!(rn.byte(), Some(0xee));
            }
            _ => assert!(false, "matching failed"),
        }

        let l3 = EncodingNode::new_leaf(0x66, 88);

        let node = EncodingNode::join(l3, node);

        let min_node = node.get_min_node();
        assert_eq!(min_node.count(), &3);

        Ok(())
    }
}

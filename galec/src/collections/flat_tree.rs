use std::collections::VecDeque;

pub type NodeId = usize;
pub const root_id: NodeId = 0;
pub const error_id: NodeId = std::usize::MAX;

pub struct FlatTree<T> {
    // (Value, is_live, parent)
    elements: Vec<(T, bool, NodeId)>
}

impl<T> FlatTree<T> {
    pub fn new_with_root(root: T) -> FlatTree<T> {
        let mut t = FlatTree{ elements: Vec::new() };
        t.elements.push((root, true, 0));
        t
    }

    pub fn new_empty() -> FlatTree<T> {
        let mut t = FlatTree{ elements: Vec::new() };
        t
    }

    pub fn set_root(&mut self, v: T) {
        match self.elements.len() {
            0 => {
                self.elements.push((v, true, 0))
            },
            _ => {
                self.elements[0] = (v, true, 0);
            }
        }
    }

    pub fn new_node(&mut self, v: T, p: NodeId) -> NodeId {
        let id = self.elements.len();
        self.elements.push((v, true, p));
        id
    }

    pub fn delete_node(&mut self, n: NodeId) {
        let mut children = Vec::new();
        match self.elements.get_mut(n).unwrap() {
            (_, ref mut v, _) if *v => {
                *v = false;
                for (id, (_, l, ref p)) in self.elements.iter().enumerate() {
                    // If is child, is live, and not the current node
                    if *p == n && *l && id != n {
                        children.push(id);
                    }
                }
            },
            _ => ()
        };

        for c in children.iter() {
            self.delete_node(*c);
        }
    }

    pub fn set_node_value(&mut self, n: NodeId, v: T) {
        match self.elements.get_mut(n).unwrap() {
            (ref mut v_old, true, _) => *v_old = v,
            _ => ()
        }
    }

    pub fn get_node_value(&self, n: NodeId) -> Option<&T> {
        match self.elements.get(n).unwrap() {
            (ref v, true, _) => Some(v),
            _ => None
        }
    }

    pub fn get_mut_node_value(&mut self, n: NodeId) -> Option<&mut T> {
        match self.elements.get_mut(n).unwrap() {
            (ref mut v, true, _) => Some(v),
            _ => None
        }
    }

    pub fn get_parent(&self, n: NodeId) -> NodeId {
        self.elements.get(n).unwrap().2
    }

    pub fn get_children(&self, n: NodeId) -> Vec<NodeId> {
        let mut r = Vec::new();
        for (id, (_, _, ref p)) in self.elements.iter().enumerate() {
            if *p == n && *p != id {
                r.push(id);
            }
        };
        r
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for FlatTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Tree [")?;
        let mut iter = self.elements.iter();
        write!(f, "{:?}", iter.next())?;
        for elem in self.elements.iter() {
            write!(f, ", {:?}", elem)?;
        };
        write!(f, "]")?;
        Ok(())
    }
}

pub struct RandomFlatTreeIterator<'a, T>{
    tree_iter: std::iter::Enumerate<std::slice::Iter<'a, (T, bool, NodeId)>>,
}

impl<'a, T> RandomFlatTreeIterator<'a, T> {
    pub fn new(t: &'a FlatTree<T>) -> RandomFlatTreeIterator<'a, T> {
        RandomFlatTreeIterator { tree_iter: t.elements.iter().enumerate() } 
    }
}

impl<'a, T> Iterator for RandomFlatTreeIterator<'a, T> {
    type Item = (NodeId, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&n) = self.tree_iter.next().as_ref() {
            match n {
                (id, (ref v, true, _)) => return Some((id, &v)),
                _ => ()
            }
        };

        None
    }
}


pub struct PreOrderTreeIterator<'a, T>{
    queue: VecDeque<NodeId>,
    tree: &'a FlatTree<T>
}

impl<'a, T> PreOrderTreeIterator<'a, T> {
    pub fn new(t: &'a FlatTree<T>) -> PreOrderTreeIterator<'a, T> {
        let mut queue = VecDeque::new();
        queue.push_front(root_id);
        PreOrderTreeIterator { queue: queue, tree: t } 
    }
}

impl<'a, T> Iterator for PreOrderTreeIterator<'a, T> {
    type Item = (NodeId, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            return None;
        }

        let next = self.queue.pop_front().unwrap();
        let children = self.tree.get_children(next);
        for child in children.iter() {
            self.queue.push_front(*child);
        }

        Some((next, self.tree.get_node_value(next).unwrap()))
    }
}


#[test]
fn test_single_child() {
    let mut t: FlatTree<i32> = FlatTree::new_with_root(3);
    let n1 = t.new_node(4, root_id);
    assert_eq!(t.get_node_value(root_id), Some(&3));
    assert_eq!(t.get_node_value(n1), Some(&4));
    assert_eq!(t.get_children(root_id), vec![n1]);
}


#[test]
fn test_multiple_children() {
    let mut t: FlatTree<i32> = FlatTree::new_with_root(3);
    let n1 = t.new_node(4, root_id);
    let n2 = t.new_node(5, root_id);
    assert_eq!(t.get_node_value(root_id), Some(&3));
    assert_eq!(t.get_node_value(n1), Some(&4));
    assert_eq!(t.get_node_value(n2), Some(&5));
    assert_eq!(t.get_children(root_id), vec![n1, n2]);

    let n3 = t.new_node(6, n2);
    assert_eq!(t.get_node_value(n3), Some(&6));
    assert_eq!(t.get_children(n2), vec![n3]);

    t.delete_node(n2);
    assert_eq!(t.get_node_value(n2), None);
    assert_eq!(t.get_node_value(n3), None);
}

// These tests should not assert the explicit value of the NodeId,
// Instead they should test that recieving the value of that NodeId is equal to the value returned in the iter result


#[test]
fn test_iterate_single() {
    let t: FlatTree<i32> = FlatTree::new_with_root(3);
    let mut it = RandomFlatTreeIterator::new(&t);
    assert_eq!(it.next(), Some((0, &3)));
    assert_eq!(it.next(), None);
}

#[test]
fn test_iterate_multiple() {
    let mut t: FlatTree<i32> = FlatTree::new_with_root(3);
    t.new_node(5, root_id);

    let mut it = RandomFlatTreeIterator::new(&t);
    assert_eq!(it.next(), Some((0, &3)));
    assert_eq!(it.next(), Some((1, &5)));
    assert_eq!(it.next(), None);
}

#[test]
fn test_iterate_with_deleted() {
    let mut t: FlatTree<i32> = FlatTree::new_with_root(3);
    let id = t.new_node(5, root_id);
    t.new_node(6, root_id);
    t.delete_node(id);

    let mut it = RandomFlatTreeIterator::new(&t);
    assert_eq!(it.next(), Some((0, &3)));
    assert_eq!(it.next(), Some((2, &6)));
    assert_eq!(it.next(), None);
}


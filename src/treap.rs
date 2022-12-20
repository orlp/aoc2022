use std::cmp::Ordering;

use rand::Rng;
use slotmap::{new_key_type, Key, SlotMap};

new_key_type! { pub struct NodeKey; }
type NodeMap<T> = SlotMap<NodeKey, TreapNode<T>>;

struct TreapNode<T> {
    value: T,
    priority: u32,
    left: NodeKey,
    right: NodeKey,
    parent: NodeKey,
    count: usize,
}

#[derive(Default)]
pub struct Treap<T> {
    nm: NodeMap<T>,
    root: NodeKey,
}

impl<T> Treap<T> {
    fn count(&self, node: NodeKey) -> usize {
        self.nm.get(node).map(|n| n.count).unwrap_or(0)
    }

    fn update(&mut self, node: NodeKey) {
        let TreapNode { left, right, .. } = self.nm[node];
        let mut count = 1;
        if let Some(l) = self.nm.get_mut(left) {
            l.parent = node;
            count += l.count;
        }
        if let Some(r) = self.nm.get_mut(right) {
            r.parent = node;
            count += r.count;
        }
        self.nm[node].count = count;
    }

    fn split(&mut self, node: NodeKey, rank: usize) -> (NodeKey, NodeKey) {
        if node.is_null() {
            return (NodeKey::null(), NodeKey::null());
        }

        let TreapNode { left, right, .. } = self.nm[node];
        let left_count = self.nm.get(left).map(|n| n.count).unwrap_or(0);
        if rank <= left_count {
            let (ll, lr) = self.split(left, rank);
            self.nm[node].left = lr;
            self.update(node);
            (ll, node)
        } else {
            let (rl, rr) = self.split(right, rank - left_count - 1);
            self.nm[node].right = rl;
            self.update(node);
            (node, rr)
        }
    }

    fn merge(&mut self, left: NodeKey, right: NodeKey) -> NodeKey {
        match (self.nm.get(left), self.nm.get(right)) {
            (Some(l), Some(r)) => {
                if l.priority < r.priority {
                    self.nm[left].right = self.merge(l.right, right);
                    self.update(left);
                    left
                } else {
                    self.nm[right].left = self.merge(left, r.left);
                    self.update(right);
                    right
                }
            },
            (None, Some(_)) => right,
            _ => left,
        }
    }

    #[inline]
    pub fn get(&self, node: NodeKey) -> Option<&T> {
        Some(&self.nm.get(node)?.value)
    }

    pub fn rank(&self, node: NodeKey) -> Option<usize> {
        let n = self.nm.get(node)?;
        let mut rank = self.count(n.left);
        let mut cur = n.parent;
        let mut prev = node;
        while let Some(c) = self.nm.get(cur) {
            if prev == c.right {
                rank += 1 + self.count(c.left);
            }
            (prev, cur) = (cur, c.parent);
        }

        Some(rank)
    }

    pub fn derank(&self, mut rank: usize) -> NodeKey {
        let mut cur = self.root;
        while let Some(c) = self.nm.get(cur) {
            let left_count = self.count(c.left);
            match rank.cmp(&self.count(c.left)) {
                Ordering::Less => cur = c.left,
                Ordering::Equal => return cur,
                Ordering::Greater => {
                    cur = c.right;
                    rank -= left_count + 1;
                },
            }
        }
        cur
    }

    pub fn insert<R: Rng>(&mut self, value: T, rank: usize, rng: &mut R) -> NodeKey {
        let (l, r) = self.split(self.root, rank);
        let m = self.nm.insert(TreapNode {
            value,
            priority: rng.gen(),
            left: NodeKey::null(),
            right: NodeKey::null(),
            parent: NodeKey::null(),
            count: 1,
        });
        let lm = self.merge(l, m);
        self.root = self.merge(lm, r);
        self.nm[self.root].parent = NodeKey::null();
        m
    }

    pub fn remove(&mut self, node: NodeKey) -> Option<(T, usize)> {
        let r = self.nm.remove(node)?;

        // Compute rank and update parent counts.
        let mut rank = self.count(r.left);
        let mut cur = r.parent;
        let mut prev = node;
        while let Some(c) = self.nm.get_mut(cur) {
            let (l, r, p) = (c.left, c.right, c.parent);
            c.count -= 1;
            if prev == r {
                rank += 1 + self.count(l);
            }
            (prev, cur) = (cur, p);
        }

        // Update parent pointers / pointers in parent.
        let merged = self.merge(r.left, r.right);
        if let Some(m) = self.nm.get_mut(merged) {
            m.parent = r.parent;
        }
        if let Some(p) = self.nm.get_mut(r.parent) {
            if p.left == node {
                p.left = merged;
            } else {
                p.right = merged;
            }
        } else {
            self.root = merged;
        }

        Some((r.value, rank))
    }
}

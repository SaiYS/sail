use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

const ALPHA_DEFAULT: f64 = 0.7;

#[derive(Debug, Default, Clone)]
pub struct ScapegoatNode<T> {
    pub value: T,
    left: RefCell<Option<Rc<ScapegoatNode<T>>>>,
    right: RefCell<Option<Rc<ScapegoatNode<T>>>>,
    parent: RefCell<Option<Weak<ScapegoatNode<T>>>>,
}

impl<T: Ord + Debug> ScapegoatNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: RefCell::new(None),
            right: RefCell::new(None),
            parent: RefCell::new(None),
        }
    }

    fn size(&self) -> usize {
        1 + self.left.borrow().as_deref().map_or(0, |left| left.size())
            + self
                .right
                .borrow()
                .as_deref()
                .map_or(0, |right| right.size())
    }

    fn height(&self) -> usize {
        1 + std::cmp::max(
            self.left
                .borrow()
                .as_deref()
                .map_or(0, |left| left.height()),
            self.right
                .borrow()
                .as_deref()
                .map_or(0, |right| right.height()),
        )
    }

    fn contains(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            std::cmp::Ordering::Less => self
                .left
                .borrow()
                .as_deref()
                .map_or(false, |left| left.contains(value)),
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Greater => self
                .right
                .borrow()
                .as_deref()
                .map_or(false, |right| right.contains(value)),
        }
    }

    fn to_sorted_vec(this: Rc<Self>) -> Vec<Rc<Self>> {
        let mut res = Vec::new();
        let mut left = this.left.borrow().as_ref().map_or(Vec::new(), |left| {
            ScapegoatNode::to_sorted_vec(Rc::clone(left))
        });
        let mut right = this.right.borrow().as_ref().map_or(Vec::new(), |right| {
            ScapegoatNode::to_sorted_vec(Rc::clone(right))
        });

        res.append(&mut left);
        res.push(this);
        res.append(&mut right);
        res
    }

    fn rebuild_inner(sorted: &[Rc<Self>], from: usize, to: usize) -> Option<Rc<Self>> {
        if from == to {
            None
        } else {
            let m = (from + to) / 2;
            let median = Rc::clone(&sorted[m]);

            dbg!(from, to, m);

            {
                *median.left.borrow_mut() = Self::rebuild_inner(sorted, from, m);
                if let Some(left) = median.left.borrow().as_deref() {
                    *left.parent.borrow_mut() = Some(Rc::downgrade(&median));
                }
            }
            {
                *median.right.borrow_mut() = Self::rebuild_inner(sorted, m + 1, to);
                if let Some(right) = median.right.borrow().as_deref() {
                    *right.parent.borrow_mut() = Some(Rc::downgrade(&median));
                }
            }
            Some(median)
        }
    }

    fn rebuild(scapegoat: Rc<Self>) {
        let sorted = Self::to_sorted_vec(Rc::clone(&scapegoat));
        match scapegoat.parent.borrow().as_ref().and_then(|p| p.upgrade()) {
            Some(parent) => {
                if parent
                    .left
                    .borrow()
                    .as_ref()
                    .map_or(false, |l| Rc::ptr_eq(l, &scapegoat))
                {
                    *parent.left.borrow_mut() = Self::rebuild_inner(&sorted, 0, sorted.len());
                } else {
                    *parent.right.borrow_mut() = Self::rebuild_inner(&sorted, 0, sorted.len());
                }
            }
            None => {
                Self::rebuild_inner(&sorted, 0, sorted.len());
            }
        }
    }

    fn __verify_ordering(&self) -> bool {
        self.left.borrow().as_deref().map_or(true, |left| {
            left.value <= self.value && left.__verify_ordering()
        }) && self.right.borrow().as_deref().map_or(true, |right| {
            self.value <= right.value && right.__verify_ordering()
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct ScapegoatTree<T> {
    len: usize,
    root: Option<Rc<ScapegoatNode<T>>>,
}

impl<T: Ord + Debug> ScapegoatTree<T> {
    pub fn new() -> Self {
        Self { len: 0, root: None }
    }

    pub fn size(&self) -> usize {
        self.root.as_ref().map_or(0, |root| root.size())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn height(&self) -> usize {
        self.root.as_deref().map_or(0, |root| root.height())
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.root
            .as_deref()
            .map_or(false, |root| root.contains(value))
    }

    pub fn insert(&mut self, value: T) {
        let mut current_node = self.root.clone();
        let new_node = Rc::new(ScapegoatNode::new(value));

        if current_node.is_none() {
            self.root = Some(new_node);
            self.len += 1;
        } else {
            let mut depth = 1usize;
            let mut descending;

            while let Some(ref cur) = current_node {
                match new_node.value.cmp(&cur.value) {
                    std::cmp::Ordering::Less => {
                        let left = cur.left.borrow().clone();
                        match left {
                            Some(left) => descending = Some(Rc::clone(&left)),
                            None => {
                                *cur.left.borrow_mut() = Some(Rc::clone(&new_node));
                                *new_node.parent.borrow_mut() = Some(Rc::downgrade(cur));
                                descending = None;
                            }
                        }
                    }
                    std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                        let right = cur.right.borrow().clone();
                        match right {
                            Some(right) => descending = Some(Rc::clone(&right)),
                            None => {
                                *cur.right.borrow_mut() = Some(Rc::clone(&new_node));
                                *new_node.parent.borrow_mut() = Some(Rc::downgrade(cur));
                                descending = None;
                            }
                        }
                    }
                }

                current_node = descending;
                depth += 1;
            }

            self.len += 1;

            if depth as f64 > ((self.len as f64).log(1. / ALPHA_DEFAULT).floor() + 1.) {
                let mut cur = new_node;
                let mut a = 1;
                let mut b;
                while let Some(parent) = Rc::clone(&cur)
                    .parent
                    .borrow()
                    .as_ref()
                    .and_then(|p| p.upgrade())
                {
                    let sibling = if Rc::clone(&parent)
                        .right
                        .borrow()
                        .as_ref()
                        .map_or(false, |r| Rc::ptr_eq(r, &cur))
                    {
                        parent.left.borrow().as_ref().map_or(0, |l| l.size())
                    } else {
                        parent.right.borrow().as_ref().map_or(0, |l| l.size())
                    };

                    b = a;
                    a = b + 1 + sibling;

                    if sibling.max(b) as f64 <= a as f64 * ALPHA_DEFAULT {
                        cur = Rc::clone(&parent);
                    } else {
                        // eprintln!("scapegoat is {:?}", &parent.value);
                        ScapegoatNode::rebuild(parent);
                        return;
                    }
                }

                unreachable!("there should be one or more scapegoat")
            }
        }
    }

    fn __verify_ordering(&self) -> bool {
        self.root
            .as_ref()
            .map_or(true, |root| root.__verify_ordering())
    }
}

#[test]
fn debug() {
    // use rand::Rng;

    // let mut rng = rand::thread_rng();
    let mut tree = ScapegoatTree::<i32>::new();
    tree.insert(0);
    tree.insert(1);
    tree.insert(2);
    tree.insert(3);
    tree.insert(4);
    tree.insert(5);
    tree.insert(6);
    tree.insert(7);

    // dbg!(tree.height());
    // dbg!(tree.size(), tree.len());
    assert!(tree.root.unwrap().__verify_ordering());

    panic!()
}

use algebraics::abstract_type::Monoid;
use itertools::Itertools;
use std::{
    fmt::{Debug, Display},
    ops::RangeBounds,
};

/// Generic segment-tree
#[derive(Clone)]
pub struct SegmentTree<M: Monoid> {
    len: usize,
    capacity: usize,
    size: usize,
    height: usize,
    buffer: Vec<M::I>,
}

impl<M> Debug for SegmentTree<M>
where
    M: Monoid,
    M::I: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = (0..self.height())
            .map(|x| {
                ((1 << x) - 1..)
                    .take(1 << x)
                    .map(|x| &self.buffer[x])
                    .join(" ")
            })
            .join("\n");
        write!(f, "\n{}", s)
    }
}

impl<M: Monoid> From<Vec<M::I>> for SegmentTree<M> {
    /// Complexity: O(n)
    fn from(v: Vec<M::I>) -> Self {
        let len = v.len();
        let capacity = len.next_power_of_two();
        let height = capacity.trailing_zeros() as usize + 1;
        let size = capacity * 2 - 1;
        let mut buffer = vec![<M as Monoid>::identity(); size];

        for (i, e) in v.into_iter().enumerate() {
            buffer[size / 2 + i] = e;
        }

        for i in (0..capacity - 1).rev() {
            buffer[i] = M::operate(buffer[i * 2 + 1].clone(), buffer[i * 2 + 2].clone());
        }

        Self {
            len,
            capacity,
            size,
            height,
            buffer,
        }
    }
}

impl<M: Monoid> From<&[M::I]> for SegmentTree<M> {
    fn from(v: &[M::I]) -> Self {
        let len = v.len();
        let capacity = len.next_power_of_two();
        let height = capacity.trailing_zeros() as usize + 1;
        let size = capacity * 2 - 1;
        let mut buffer = vec![<M as Monoid>::identity(); size];

        for (i, e) in v.iter().enumerate() {
            buffer[size / 2 + i] = e.clone();
        }

        for i in (0..capacity - 1).rev() {
            buffer[i] = M::operate(buffer[i * 2 + 1].clone(), buffer[i * 2 + 2].clone());
        }

        Self {
            len,
            capacity,
            size,
            height,
            buffer,
        }
    }
}

impl<M: Monoid> SegmentTree<M> {
    /// Returns the size of its buffer
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the length of the original array, NOT size of its buffer
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the height of the tree
    pub fn height(&self) -> usize {
        self.height
    }

    /// Create a new empty SegmentTree with given length
    pub fn new(len: usize) -> Self {
        let capacity = len.next_power_of_two();
        let height = capacity.trailing_zeros() as usize + 1;
        let size = capacity * 2 - 1;
        Self {
            len,
            capacity,
            size,
            height,
            buffer: vec![<M as Monoid>::identity(); size],
        }
    }

    /// Returns ref of original array sliced from its buffer
    pub fn raw_leaves(&self) -> &[M::I] {
        &self.buffer[self.capacity - 1..self.size]
    }

    /// Returns a value of i-th leaf
    ///
    /// Complexity: O(1)
    pub fn get(&self, i: usize) -> M::I {
        self.buffer[self.capacity - 1 + i].clone()
    }

    /// Returns a folded value of leaves in `range`
    ///
    /// Complexity: O(log n)
    pub fn range<R: RangeBounds<usize>>(&self, range: R) -> M::I {
        let (from, to) = util::expand_range_bound(&range, 0, self.len());
        debug_assert!(from < to);

        let mut from = from + self.capacity - 1;
        let mut to = to + self.capacity - 1;

        let mut ls = <M as Monoid>::identity();
        let mut rs = <M as Monoid>::identity();

        while from < to {
            if from & 1 == 0 {
                M::operate_assign(&mut ls, self.buffer[from].clone());
                from += 1;
            }
            if to & 1 == 0 {
                to -= 1;
                M::operate_assign(&mut rs, self.buffer[to].clone());
            }
            from = (from - 1) >> 1;
            to = (to - 1) >> 1;
        }

        M::operate(ls, rs)
    }

    /// Returns a folded value of all leaves
    ///
    /// Complexity: O(1)
    /// This can be more efficient than calling `self.get_range(..)`
    pub fn all(&self) -> M::I {
        self.buffer[0].clone()
    }

    /// Update one value at index `i` with `new_value`
    ///
    /// Complexity: O(log n)
    pub fn update(&mut self, i: usize, new_value: M::I) {
        let mut cur = self.capacity - 1 + i;
        self.buffer[cur] = new_value;
        while cur != 0 {
            cur = (cur - 1) >> 1;
            self.buffer[cur] = M::operate(
                self.buffer[cur * 2 + 1].clone(),
                self.buffer[cur * 2 + 2].clone(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentTree;
    use algebraics::{abstract_type::Monoid, structure::Min};
    use itertools::Itertools;
    use rand::Rng;

    fn verify() {
        let mut rng = rand::thread_rng();

        let n = 1000usize;
        let a = (0..n).map(|_| rng.gen_range(0..n)).collect_vec();

        let mut st = SegmentTree::<Min<usize>>::from(a);

        for _ in 0..n {
            if rng.gen_bool(0.8) {
                // query
                let mut from = rng.gen_range(0..n);
                let mut to = rng.gen_range(0..n);

                if from > to {
                    std::mem::swap(&mut from, &mut to);
                }

                assert_eq!(
                    st.range(from..=to),
                    Min::fold_right(&st.raw_leaves()[from..=to])
                );
            } else {
                // update
                let i = rng.gen_range(0..n);
                let value = rng.gen_range(0..n);
                st.update(i, value);
            }
        }
    }

    #[test]
    fn run_varify() {
        for _ in 0..100 {
            verify();
        }
    }
}

#[allow(dead_code)]
mod another {
    use std::{cell::RefCell, rc::Rc};

    #[derive(Debug)]
    pub struct SegmentTree<T, F> {
        tree: Option<Rc<RefCell<SegmentTreeNode<T>>>>,
        op: F,
        neutral: T,
    }

    impl<T: Copy, F: Fn(T, T) -> T> SegmentTree<T, F> {
        fn new(v: &[T], op: F, neutral: T) -> Self {
            Self {
                tree: SegmentTreeNode::new_inner(v, (0, v.len()), &op),
                op,
                neutral,
            }
        }

        fn get(&self, range: (usize, usize)) -> T {
            self.tree.as_ref().map_or(self.neutral, |tree| {
                tree.borrow().get_inner(range, &self.op, self.neutral)
            })
        }

        fn update(&mut self, i: usize, to: T) {
            if let Some(tree) = self.tree.as_mut() {
                tree.borrow_mut().update(i, to, &self.op, self.neutral);
            }
        }
    }

    #[derive(Debug)]
    pub struct SegmentTreeNode<T> {
        value: T,
        range: (usize, usize),
        left: Option<Rc<RefCell<SegmentTreeNode<T>>>>,
        right: Option<Rc<RefCell<SegmentTreeNode<T>>>>,
    }

    impl<T: Copy> SegmentTreeNode<T> {
        fn new_inner<F: Fn(T, T) -> T>(
            v: &[T],
            range: (usize, usize),
            op: &F,
        ) -> Option<Rc<RefCell<Self>>> {
            let l = v.len();

            if l == 0 {
                None
            } else if l == 1 {
                Some(Rc::new(RefCell::new(Self {
                    value: v[0],
                    range,
                    left: None,
                    right: None,
                })))
            } else {
                let t = l / 2;
                let left = Self::new_inner(&v[..t], (range.0, range.0 + t), op);
                let right = Self::new_inner(&v[t..], (range.0 + t, range.1), op);

                let value = match (&left, &right) {
                    (None, None) => todo!(),
                    (None, Some(r)) => r.borrow().value,
                    (Some(l), None) => l.borrow().value,
                    (Some(l), Some(r)) => op(l.borrow().value, r.borrow().value),
                };

                Some(Rc::new(RefCell::new(Self {
                    value,
                    range,
                    left,
                    right,
                })))
            }
        }

        fn get_inner<F: Fn(T, T) -> T>(&self, range: (usize, usize), op: &F, neutral: T) -> T {
            if range.1 <= self.range.0 || self.range.1 <= range.0 {
                neutral
            } else if range.0 <= self.range.0 && self.range.1 <= range.1 {
                self.value
            } else {
                op(
                    self.left
                        .as_ref()
                        .map_or(neutral, |left| left.borrow().get_inner(range, op, neutral)),
                    self.right.as_ref().map_or(neutral, |right| {
                        right.borrow().get_inner(range, op, neutral)
                    }),
                )
            }
        }

        fn update<F: Fn(T, T) -> T>(&mut self, i: usize, to: T, op: &F, neutral: T) {
            if self.range.0 == i && self.range.1 == i + 1 {
                self.value = to;
            } else if i < self.range.0 || self.range.1 <= i {
                // do nothing
            } else if self.range.0 <= i && i < self.range.1 {
                self.value = op(
                    self.left.as_mut().map_or(neutral, |left| {
                        (*left.borrow_mut()).update(i, to, op, neutral);
                        left.borrow().value
                    }),
                    self.right.as_mut().map_or(neutral, |right| {
                        right.borrow_mut().update(i, to, op, neutral);
                        right.borrow().value
                    }),
                );
            }
        }
    }
}

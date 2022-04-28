use crate::Monoid;

/// See https://en.wikipedia.org/wiki/Semigroup_action#S-Act_and_M-Act
pub trait MonoidAction: Monoid {
    type X: Monoid;
    fn act(m: <Self as Monoid>::I, x: <Self::X as Monoid>::I) -> <Self::X as Monoid>::I;
}

pub struct DelayedSegmentTree<M, A>
where
    M: Monoid,
    A: MonoidAction<X = M>,
{
    len: usize,
    capacity: usize,
    size: usize,
    height: usize,
    buffer: Vec<M::I>,
    actor: Vec<A::I>,
}

impl<M, A> DelayedSegmentTree<M, A>
where
    M: Monoid,
    A: MonoidAction<X = M>,
{
    pub fn new(len: usize) -> Self {
        let capacity = len.next_power_of_two();
        let height = capacity.trailing_zeros() as usize + 1;
        let size = capacity * 2 - 1;
        let buffer = vec![<M as Monoid>::identity(); size];
        let actor = vec![<A as Monoid>::identity(); size];
        Self {
            len,
            capacity,
            size,
            height,
            buffer,
            actor,
        }
    }

    /// Returns ref of original array sliced from its buffer
    pub fn raw_leaves(&self) -> &[M::I] {
        &self.buffer[self.capacity - 1..self.size]
    }

    fn reflect(&mut self, i: usize) {
        let mut action = <A as Monoid>::identity();
        std::mem::swap(&mut self.actor[i], &mut action);

        self.buffer[i] = A::act(action.clone(), self.buffer[i].clone());
        A::operate_assign(&mut self.actor[i * 2 + 1], action.clone());
        A::operate_assign(&mut self.actor[i * 2 + 2], action);
    }
}

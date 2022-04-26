use crate::Monoid;

/// See https://en.wikipedia.org/wiki/Semigroup_action#S-Act_and_M-Act
pub trait MonoidAction {
    type M: Monoid;
    type X: Monoid;
    fn act(m: Self::M, x: Self::X) -> Self::X;
}

pub struct DelayedSegmentTree<MX, A>
where
    MX: MonoidAction,
    A: Fn(MX::M, MX::X) -> MX::X,
{
    len: usize,
    capacity: usize,
    size: usize,
    height: usize,
    buffer: Vec<<MX::X as Monoid>::I>,
    actor: Vec<<MX::M as Monoid>::I>,
    action: A,
}

impl<MX, A> DelayedSegmentTree<MX, A>
where
    MX: MonoidAction,
    A: Fn(MX::M, MX::X) -> MX::X,
{
    pub fn new(len: usize, action: A) -> Self {
        let capacity = len.next_power_of_two();
        let height = capacity.trailing_zeros() as usize + 1;
        let size = capacity * 2 - 1;
        let buffer = vec![<MX::X as Monoid>::identity(); size];
        let actor = vec![<MX::M as Monoid>::identity(); size];
        Self {
            len,
            capacity,
            size,
            height,
            buffer,
            actor,
            action,
        }
    }
}

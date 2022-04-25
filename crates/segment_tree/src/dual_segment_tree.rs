// use algebraics::monoid::Monoid;

// pub struct DualSegmentTree<M: Monoid> {
//     len: usize,
//     capacity: usize,
//     size: usize,
//     height: usize,
//     buffer: Vec<M>,
// }

// impl<M: Monoid> DualSegmentTree<M> {
//     /// Create a new empty DualSegmentTree with given length
//     pub fn new(len: usize) -> Self {
//         let capacity = len.next_power_of_two();
//         let height = capacity.trailing_zeros() as usize + 1;
//         let size = capacity * 2 - 1;
//         Self {
//             len,
//             capacity,
//             size,
//             height,
//             buffer: vec![M::identity(); size],
//         }
//     }

//     /// Returns ref of original array sliced from its buffer
//     pub fn raw_leaves(&self) -> &[M] {
//         &self.buffer[self.capacity - 1..self.size]
//     }

//     /// Returns a value of i-th leaf
//     ///
//     /// Complexity: O(1)
//     pub fn get(&self, i: usize) -> M::T {
//         let mut cur = self.capacity - 1 + i;
//         let mut res = self.buffer[cur].clone();

//         while cur != 0 {
//             cur = (cur - 1) >> 1;
//             res = M::binary_operation(res.clone(), self.buffer[cur].clone());
//         }

//         res.get()
//     }

//     // pub fn update(&mut self, range: R, ) {}
// }

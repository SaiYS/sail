use std::ops::Add;
pub use unweighted::{
    list_graph::{DULGraph, UULGraph},
    matrix_graph::{DUMGraph, UUMGraph},
};

pub mod unweighted {
    pub mod list_graph;
    pub mod matrix_graph;
}

pub mod weighted {
    pub mod list_graph;
    pub mod matrix_graph;
}

pub use weighted::{
    list_graph::{DWLGraph, UWLGraph},
    matrix_graph::{DWMGraph, UWMGraph},
};

pub mod bellman_ford;
pub mod dijkstra;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Distance<T> {
    Reachable(T),
    Unreachable,
}

impl<T> Distance<T> {
    pub fn is_unreachable(self) -> bool {
        matches!(self, Distance::Unreachable)
    }

    pub fn is_reachable(self) -> bool {
        matches!(self, Distance::Reachable(_))
    }

    pub fn unwrap(self) -> T {
        match self {
            Distance::Reachable(x) => x,
            Distance::Unreachable => panic!("Distance::unwrap at Unreachable"),
        }
    }
}

impl<T: Add<Output = T>> Add for Distance<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Distance::Reachable(x), Distance::Reachable(y)) => Distance::Reachable(x + y),
            (Distance::Reachable(_), Distance::Unreachable) => Distance::Unreachable,
            (Distance::Unreachable, Distance::Reachable(_)) => Distance::Unreachable,
            (Distance::Unreachable, Distance::Unreachable) => Distance::Unreachable,
        }
    }
}

impl<T: Add<Output = T>> Add<T> for Distance<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        match self {
            Distance::Reachable(x) => Distance::Reachable(x + rhs),
            Distance::Unreachable => Distance::Unreachable,
        }
    }
}

impl<T: Copy> Iterator for Distance<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Distance::Reachable(x) => Some(x),
            Distance::Unreachable => None,
        }
    }
}

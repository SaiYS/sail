#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directed {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Undirected {}

pub trait DirectionType {
    const DIRECTED: bool;
}

impl DirectionType for Directed {
    const DIRECTED: bool = true;
}

impl DirectionType for Undirected {
    const DIRECTED: bool = false;
}

pub mod list_graph;
pub mod matrix_graph;

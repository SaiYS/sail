pub mod unweighted {
    pub mod list_graph;
    pub mod matrix_graph;
}

pub mod weighted {
    pub mod list_graph;
    pub mod matrix_graph;
}

pub use unweighted::{
    list_graph::{DULGraph, UULGraph},
    matrix_graph::{DUMGraph, UUMGraph},
};

pub use weighted::{
    list_graph::{DWLGraph, UWLGraph},
    matrix_graph::{DWMGraph, UWMGraph},
};

use std::collections::HashMap;

use itertools::Itertools;

pub struct IndexCompression {
    com: HashMap<usize, usize>,
    dec: Vec<usize>,
}

impl IndexCompression {
    pub fn new(indices: &[usize]) -> Self {
        let mut compress = HashMap::new();
        let mut decompress = Vec::new();

        for (&decomp, comp) in indices.iter().sorted().enumerate().map(|(a, b)| (b, a)) {
            compress.insert(decomp, comp);
            decompress.push(decomp);
        }

        Self {
            com: compress,
            dec: decompress,
        }
    }

    pub fn compress(&self, from: usize) -> usize {
        self.com[&from]
    }

    pub fn decompress(&self, compressed: usize) -> usize {
        self.dec[compressed]
    }
}

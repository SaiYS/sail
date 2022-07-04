use std::collections::HashMap;

use itertools::Itertools;

pub struct IndexCompression {
    len: usize,
    com: HashMap<usize, usize>,
    dec: Vec<usize>,
}

impl IndexCompression {
    pub fn new(indices: &[usize]) -> Self {
        let mut compress = HashMap::new();
        let mut decompress = Vec::new();

        for (&decomp, comp) in indices
            .iter()
            .unique()
            .sorted()
            .enumerate()
            .map(|(a, b)| (b, a))
        {
            compress.insert(decomp, comp);
            decompress.push(decomp);
        }

        Self {
            len: decompress.len(),
            com: compress,
            dec: decompress,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn compress(&self, from: usize) -> Option<usize> {
        self.com.get(&from).copied()
    }

    pub fn decompress(&self, compressed: usize) -> usize {
        self.dec[compressed]
    }
}

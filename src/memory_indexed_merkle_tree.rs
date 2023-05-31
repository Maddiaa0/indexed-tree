use ruint::Uint;

use crate::{
    hash::hash_pair,
    leaf::{IndexedTreeLeaf, NullifierLeaf},
};

pub struct MemoryIndexedMerkleTree {
    pub depth: u64,
    pub total_size: u64,
    pub leaves: Vec<NullifierLeaf>,
    pub hashes: Vec<Uint<256, 4>>,
    pub root: Uint<256, 4>,
}

// Note this builds and stores the whole tree in memory, useful for testing with small trees, but will not scale
impl MemoryIndexedMerkleTree {
    pub fn new(depth: u64) -> Self {
        // Limit tree size as it is in memory
        let total_size = 1 << depth;
        let mut leaves: Vec<NullifierLeaf> = Vec::with_capacity(total_size); // store just leaves
        let mut hashes: Vec<Uint<256, 4>> = Vec::with_capacity(total_size * 2 - 2); // store every node in the tree

        let mut current = NullifierLeaf::empty().hash();

        // Build tree with empty leaves
        let mut offset: usize = 0;
        let mut layer_size = total_size;
        while offset < total_size {
            for i in 0..layer_size {
                hashes[offset + i] = current;
            }
            current = hash_pair(current, current);

            offset += layer_size;
            layer_size >>= 1;
        }

        // TODO: less yucky cloney
        let initial_leaf = NullifierLeaf::new(Some(IndexedTreeLeaf::empty()));
        leaves.push(initial_leaf.clone());

        let mut tree = Self {
            depth,
            total_size: total_size as u64,
            leaves,
            hashes,
            root: current,
        };
        tree.update_element(0, initial_leaf);
        tree
    }

    fn update_element(&mut self, index: u64, leaf: NullifierLeaf) {
        let mut idx = index;
        let mut offset = 0;
        let mut layer_size = self.total_size;

        let mut current = leaf.hash();

        for mut i in 0..self.depth {
            self.hashes[(offset + idx) as usize] = current;
            idx &= !0u64 - 1;
            current = hash_pair(
                self.hashes[(offset + idx) as usize],
                self.hashes[(offset + idx + 1) as usize],
            );
            offset += layer_size;
            layer_size >>= 1;
            idx >>= 1;
        }
    }

    pub fn insert(&mut self, leaf: Uint<256, 4>) {}

    // TODO: less yuck
    pub fn get_hash_path(&self, index: u64) -> Vec<[Uint<256, 4>; 2]> {
        let mut path: Vec<[Uint<256, 4>; 2]> = Vec::with_capacity(self.depth as usize);

        let mut idx = index;
        let mut offset = 0;
        let mut layer_size = self.total_size;
        for i in 0..self.depth {
            idx -= idx & 1;

            path[i as usize] = [
                self.hashes[(offset + idx) as usize],
                self.hashes[(offset + idx + 1) as usize],
            ];

            offset += layer_size;
            layer_size >>= 1;
            idx >>= 1;
        }
        path
    }

    // NOTE: un-optimised impl, this would be indexed / sorted
    // This is o(n)
    fn find_closest_leaf(self, new_value: Uint<256, 4>) -> usize {
        let mut diff: Vec<Uint<256, 4>> = Vec::new();

        for leaf in self.leaves {
            if leaf.is_zero() {
                diff.push(new_value);
            }

            let leaf_value = leaf.inner_ref().value;
            if leaf_value > new_value {
                diff.push(new_value);
            }
            //TODO: handle repeat
            else if leaf_value < new_value {
                diff.push(new_value - leaf_value);
            }
        }

        // get min in vector
        // TODO: naughty naughty unwrappy
        let min = diff
            .iter()
            .enumerate()
            .min_by_key(|&(_, item)| item)
            .unwrap()
            .0;

        return min;
    }
}

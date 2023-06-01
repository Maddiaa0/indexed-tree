use ruint::Uint;

use crate::{
    hash::hash_pair,
    leaf::{IndexedTreeLeaf, NullifierLeaf},
};

pub struct MemoryIndexedMerkleTree {
    pub depth: u64,
    pub total_size: u64,
    pub root: Uint<256, 4>,

    pub leaves: Vec<NullifierLeaf>,
    hashes: Vec<Uint<256, 4>>,
}

// Note this builds and stores the whole tree in memory, useful for testing with small trees, but will not scale
impl MemoryIndexedMerkleTree {
    #[allow(clippy::uninit_vec)]
    pub fn new(depth: u64) -> Self {
        // Limit tree size as it is in memory
        assert!(depth <= 32);

        let total_size = 1 << depth;
        let num_nodes = total_size * 2 - 2;

        // Allocation
        let mut leaves: Vec<NullifierLeaf> = Vec::with_capacity(total_size); // store just leaves
        let mut hashes: Vec<Uint<256, 4>> = Vec::with_capacity(num_nodes); // store every node in the tree

        // where we're goin', we don't need safety
        unsafe {
            hashes.set_len(num_nodes);
        }

        // Build tree with empty leaves, and hashes
        let mut current = NullifierLeaf::empty().hash();
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
        tree.update_element(0, &initial_leaf);
        tree
    }

    pub fn size(&self) -> usize {
        self.leaves.len()
    }

    pub fn get_leaf(&self, index: u64) -> NullifierLeaf {
        self.leaves[index as usize].clone()
    }

    fn update_element(&mut self, index: u64, leaf: &NullifierLeaf) -> Uint<256, 4> {
        let mut idx = index;
        let mut offset = 0;
        let mut layer_size = self.total_size;

        let mut current = leaf.hash();

        for i in 0..self.depth {
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
        self.root = current;
        self.root
    }

    pub fn insert_element(&mut self, value: Uint<256, 4>) -> Uint<256, 4> {
        // If value is 0, we can just increase the tree size
        if value == Uint::ZERO {
            let zero_leaf = NullifierLeaf::zero();
            self.leaves.push(zero_leaf.clone());
            // TODO: sort out this casting index shite
            return self.update_element((self.leaves.len() - 1) as u64, &zero_leaf);
        }

        let closest_leaf_index = self.find_closest_leaf(&value);

        let wrapped_current_leaf = self.leaves[closest_leaf_index].clone();
        let mut current_leaf = wrapped_current_leaf.clone().inner();
        let new_leaf =
            NullifierLeaf::new_leaf(value, current_leaf.next_index, current_leaf.next_value);

        // Update the current leaf to point at the new leaf
        current_leaf.next_index = self.leaves.len();
        current_leaf.next_value = value;
        self.leaves[closest_leaf_index] = NullifierLeaf::new(Some(current_leaf));

        // Insert new leaf
        self.leaves.push(new_leaf.clone());

        // Update the old leaf in the tree
        self.update_element(closest_leaf_index as u64, &wrapped_current_leaf);

        // Insert new leaf
        let root = self.update_element((self.leaves.len() - 1) as u64, &new_leaf);
        root
    }

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
    fn find_closest_leaf(&self, new_value: &Uint<256, 4>) -> usize {
        let mut diff: Vec<Uint<256, 4>> = Vec::new();

        for leaf in &self.leaves {
            if leaf.is_zero() {
                diff.push(*new_value);
                continue;
            }

            // TODO: unclone
            let leaf_value = leaf.clone().inner().value;
            if leaf_value > *new_value {
                diff.push(*new_value);
            }
            //TODO: handle repeat
            else if leaf_value < *new_value {
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

        min
    }
}

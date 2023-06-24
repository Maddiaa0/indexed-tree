mod hash;
mod leaf;
mod memory_indexed_merkle_tree;

macro_rules! nullifier {
    ($value:expr, $next_index:expr, $next_value:expr ) => {
        NullifierLeaf::new_leaf(Uint::from($value), $next_index, Uint::from($next_value))
    };
}

/// Indexed Merkle Tree Tests
#[cfg(test)]
mod tests {
    use crate::{
        hash::hash_pair, leaf::NullifierLeaf, memory_indexed_merkle_tree::MemoryIndexedMerkleTree,
    };
    use ruint::Uint;

    // TODO: check inserting the same item again
    #[test]
    fn test_tree() {
        // Test small depth 3 tree
        let mut tree = MemoryIndexedMerkleTree::new(3);

        // Initial state:
        //
        //  index     0       1       2       3        4       5       6       7
        //  ---------------------------------------------------------------------
        //  val       0       0       0       0        0       0       0       0
        //  nextIdx   0       0       0       0        0       0       0       0
        //  nextVal   0       0       0       0        0       0       0       0
        let zero_leaf = NullifierLeaf::zero();
        // TODO: assert size
        assert_eq!(tree.size(), 1);
        assert_eq!(tree.get_leaf(0), zero_leaf);

        // Add new value 30:
        //
        //  index     0       1       2       3        4       5       6       7
        //  ---------------------------------------------------------------------
        //  val       0       30      0       0        0       0       0       0
        //  nextIdx   1       0       0       0        0       0       0       0
        //  nextVal   30      0       0       0        0       0       0       0
        tree.insert_element(Uint::from(30));

        let expected_leaf = nullifier!(30, 0, 0);
        let expected_low_nullifier = nullifier!(0, 1, 30);
        assert_eq!(tree.size(), 2);
        assert_eq!(tree.get_leaf(0).hash(), expected_low_nullifier.hash());
        assert_eq!(tree.get_leaf(1).hash(), expected_leaf.hash());

        //Add new value 10:
        //
        // index     0       1       2       3        4       5       6       7
        // ---------------------------------------------------------------------
        // val       0       30      10      0        0       0       0       0
        // nextIdx   2       0       1       0        0       0       0       0
        // nextVal   10      0       30      0        0       0       0       0
        tree.insert_element(Uint::from(10));

        assert_eq!(tree.size(), 3);
        assert_eq!(tree.get_leaf(0).hash(), nullifier!(0, 2, 10).hash());
        assert_eq!(tree.get_leaf(1).hash(), nullifier!(30, 0, 0).hash());
        assert_eq!(tree.get_leaf(2).hash(), nullifier!(10, 1, 30).hash());

        //Add new value 20:
        //
        // index     0       1       2       3        4       5       6       7
        // ---------------------------------------------------------------------
        // val       0       30      10      20       0       0       0       0
        // nextIdx   2       0       3       1        0       0       0       0
        // nextVal   10      0       20      30       0       0       0       0
        //
        tree.insert_element(Uint::from(20));

        assert_eq!(tree.size(), 4);
        assert_eq!(tree.get_leaf(0).hash(), nullifier!(0, 2, 10).hash());
        assert_eq!(tree.get_leaf(1).hash(), nullifier!(30, 0, 0).hash());
        assert_eq!(tree.get_leaf(2).hash(), nullifier!(10, 3, 20).hash());
        assert_eq!(tree.get_leaf(3).hash(), nullifier!(20, 1, 30).hash());

        // Add new value 50:
        //
        //  index     0       1       2       3        4       5       6       7
        //  ---------------------------------------------------------------------
        //  val       0       30      10      20       50      0       0       0
        //  nextIdx   2       4       3       1        0       0       0       0
        //  nextVal   10      50      20      30       0       0       0       0
        tree.insert_element(Uint::from(50));
        assert_eq!(tree.size(), 5);
        assert_eq!(tree.get_leaf(0).hash(), nullifier!(0, 2, 10).hash());
        assert_eq!(tree.get_leaf(1).hash(), nullifier!(30, 4, 50).hash());
        assert_eq!(tree.get_leaf(2).hash(), nullifier!(10, 3, 20).hash());
        assert_eq!(tree.get_leaf(3).hash(), nullifier!(20, 1, 30).hash());
        assert_eq!(tree.get_leaf(4).hash(), nullifier!(50, 0, 0).hash());

        // Check the merkle paths, we manually build the tree and check parity.
        // Naming of nodes here is n<level><index> e.g. n04 is the node at level 0, index 4

        // Level 0
        let n00 = nullifier!(0, 2, 10).hash();
        let n01 = nullifier!(30, 4, 50).hash();
        let n02 = nullifier!(10, 3, 20).hash();
        let n03 = nullifier!(20, 1, 30).hash();
        let n04 = nullifier!(50, 0, 0).hash();
        let n05 = nullifier!(0, 0, 0).hash();
        let n06 = nullifier!(0, 0, 0).hash();
        let n07 = nullifier!(0, 0, 0).hash();

        // Level 1
        let n10 = hash_pair(n00, n01);
        let n11 = hash_pair(n02, n03);
        let n12 = hash_pair(n04, n05);
        let n13 = hash_pair(n06, n07);

        // Level 2
        let n20 = hash_pair(n10, n11);
        let n21 = hash_pair(n12, n13);

        // Root
        let root = hash_pair(n20, n21);

        // Item 0
        let merkle_path_0 = tree.get_hash_path(0);
        // Path should be, from leaf to root:
        // (n00, n01), (n10, n11), (n20, n21)
        assert_eq!(merkle_path_0.len(), 3);
        assert_eq!(merkle_path_0, [[n00, n01], [n10, n11], [n20, n21]]);

        // Item 1
        let merkle_path_1 = tree.get_hash_path(1);
        // Path should be, from leaf to root:
        // (n00, n01), (n10, n11), (n20, n21)
        assert_eq!(merkle_path_1.len(), 3);
        assert_eq!(merkle_path_1, [[n00, n01], [n10, n11], [n20, n21]]);

        // Item 2
        let merkle_path_2 = tree.get_hash_path(2);
        // Path should be, from leaf to root:
        // (n02, n03), (n10, n11), (n20, n21)
        assert_eq!(merkle_path_2.len(), 3);
        assert_eq!(merkle_path_2, [[n02, n03], [n10, n11], [n20, n21]]);

        // Item 3
        let merkle_path_3 = tree.get_hash_path(3);
        // Path should be, from leaf to root:
        // (n02, n03), (n10, n11), (n20, n21)
        assert_eq!(merkle_path_3.len(), 3);
        assert_eq!(merkle_path_3, [[n02, n03], [n10, n11], [n20, n21]]);

        // Item 4
        let merkle_path_4 = tree.get_hash_path(4);
        // Path should be, from leaf to root:
        // (n04, n05), (n12, n13), (n20, n21)
        assert_eq!(merkle_path_4.len(), 3);
        assert_eq!(merkle_path_4, [[n04, n05], [n12, n13], [n20, n21]]);

        // Item 5
        let merkle_path_5 = tree.get_hash_path(5);
        // Path should be, from leaf to root:
        // (n04, n05), (n12, n13), (n20, n21)
        assert_eq!(merkle_path_5.len(), 3);
        assert_eq!(merkle_path_5, [[n04, n05], [n12, n13], [n20, n21]]);

        // Item 6
        let merkle_path_6 = tree.get_hash_path(6);
        // Path should be, from leaf to root:
        // (n06, n07), (n12, n13), (n20, n21)
        assert_eq!(merkle_path_6.len(), 3);
        assert_eq!(merkle_path_6, [[n06, n07], [n12, n13], [n20, n21]]);

        // Item 7
        let merkle_path_7 = tree.get_hash_path(7);
        // Path should be, from leaf to root:
        // (n06, n07), (n12, n13), (n20, n21)
        assert_eq!(merkle_path_7.len(), 3);
        assert_eq!(merkle_path_7, [[n06, n07], [n12, n13], [n20, n21]]);
    }
}

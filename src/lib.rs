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
    use crate::{leaf::NullifierLeaf, memory_indexed_merkle_tree::MemoryIndexedMerkleTree};
    use ruint::Uint;

    // TODO: check inserting the same item again
    #[test]
    fn test_tree() {
        // Test small depth 3 tree
        let mut tree = MemoryIndexedMerkleTree::new(3);

        // Intial state:
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
    }
}

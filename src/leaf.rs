use ruint::Uint;

// TODO maybe replace with crypto?
use sha2::{Digest, Sha256};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NullifierLeaf {
    leaf: Option<IndexedTreeLeaf>,
}

impl NullifierLeaf {
    pub fn new(leaf: Option<IndexedTreeLeaf>) -> Self {
        Self { leaf }
    }

    pub fn new_leaf(value: Uint<256, 4>, next_index: usize, next_value: Uint<256, 4>) -> Self {
        Self {
            leaf: Some(IndexedTreeLeaf::new(value, next_index, next_value)),
        }
    }

    pub fn empty() -> Self {
        Self { leaf: None }
    }

    // TODO: naming is awful
    pub fn zero() -> Self {
        Self {
            leaf: Some(IndexedTreeLeaf::empty()),
        }
    }

    pub fn inner(self) -> IndexedTreeLeaf {
        match self.leaf {
            Some(l) => l,
            None => IndexedTreeLeaf::empty(),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self.leaf {
            Some(ref leaf) => leaf.is_zero(),
            None => true,
        }
    }

    pub fn hash(&self) -> Uint<256, 4> {
        match self.leaf {
            Some(ref leaf) => {
                if leaf.is_zero() {
                    Uint::ZERO
                } else {
                    leaf.hash_leaf()
                }
            }
            None => Uint::ZERO,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedTreeLeaf {
    pub value: Uint<256, 4>,
    pub next_index: usize,
    pub next_value: Uint<256, 4>,
}

impl IndexedTreeLeaf {
    pub fn new(value: Uint<256, 4>, next_index: usize, next_value: Uint<256, 4>) -> Self {
        Self {
            value,
            next_index,
            next_value,
        }
    }

    pub fn empty() -> Self {
        Self {
            value: Uint::ZERO,
            next_index: 0,
            next_value: Uint::ZERO,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.value == Uint::ZERO
    }

    pub fn hash_leaf(&self) -> Uint<256, 4> {
        let mut hasher = Sha256::new();
        hasher.update(self.value.to_be_bytes::<32>());
        hasher.update(self.next_index.to_be_bytes());
        hasher.update(self.next_value.to_be_bytes::<32>());
        let hash_result: [u8; 32] = hasher.finalize().into();
        Uint::from_be_bytes(hash_result)
    }
}

use ruint::Uint;
use sha2::{Digest, Sha256};

pub fn hash_pair(lhs: Uint<256, 4>, rhs: Uint<256, 4>) -> Uint<256, 4> {
    let mut hasher = Sha256::new();
    hasher.update(lhs.to_be_bytes::<32>());
    hasher.update(rhs.to_be_bytes::<32>());
    let result: [u8; 32] = hasher.finalize().into();
    Uint::from_be_bytes(result)
}

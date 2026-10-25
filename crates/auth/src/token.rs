use rand::RngCore;
use sha2::{Digest, Sha256};

use crate::TokenCodec;

pub struct OpaqueTokenCodec;

impl TokenCodec for OpaqueTokenCodec {
    fn generate(&self) -> (String, Vec<u8>) {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let transport: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
        let hash = self.hash(&transport);
        (transport, hash)
    }

    fn hash(&self, token: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hasher.finalize().to_vec()
    }
}

impl std::fmt::Debug for OpaqueTokenCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpaqueTokenCodec").finish_non_exhaustive()
    }
}

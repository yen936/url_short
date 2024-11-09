use sha2::{Sha256, Digest};
use hex;

pub fn hash_url(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hex::encode(hasher.finalize())
}

pub fn generate_short_url(original_url: &str) -> String {
    let hash = hash_url(original_url);
    let slice = hash[0..8].to_string();
    "https://".to_string() + &slice
}

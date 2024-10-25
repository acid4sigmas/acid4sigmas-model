use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct Hasher;

impl Hasher {
    pub fn hash(target: &str) -> Result<String> {
        Ok(hash(target, DEFAULT_COST)?)
    }

    pub fn verify(target: &str, hashed_target: &str) -> Result<bool> {
        Ok(verify(target, hashed_target)?)
    }
}

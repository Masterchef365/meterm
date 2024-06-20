use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::io::{Write, Result};

// I think we're parked, man

#[derive(PartialEq, Eq)]
pub struct HashBySerialize<T>(pub T);

impl<T: serde::Serialize> Hash for HashBySerialize<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        bincode::serialize_into(HashWriter(state), &self.0).unwrap();
    }
}

pub struct HashWriter<T>(pub T);

impl<T: Hasher> Write for HashWriter<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Hash)]
pub struct EqByHash<T>(pub T);

impl<T: Hash> PartialEq<EqByHash<T>> for EqByHash<T> {
    fn eq(&self, other: &EqByHash<T>) -> bool {
        let mut hash_self = DefaultHasher::new();
        self.0.hash(&mut hash_self);

        let mut hash_other = DefaultHasher::new();
        other.0.hash(&mut hash_other);

        hash_self.finish() == hash_other.finish()
    }
}

impl<T: Hash> Eq for EqByHash<T> {}

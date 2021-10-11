use anyhow::Result;

use crate::types::*;

mod memory_storage;

#[cfg(feature = "memory-storage")]
pub use memory_storage::MemoryStorage as Storage;

pub trait StorageBackend {
    fn enqueue(&mut self, id: Identifier, value: Value) -> Result<()>;
    fn dequeue(&mut self, id: Identifier) -> Result<Value>;
    fn length(&self, id: Identifier) -> Result<usize>;
    fn peek(&self, id: Identifier) -> Result<&Value>;
}

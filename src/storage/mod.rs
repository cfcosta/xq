use anyhow::Result;

use crate::types::*;

#[cfg(feature = "memory-storage")]
mod memory;
#[cfg(feature = "memory-storage")]
pub use memory::MemoryStorage as Storage;
#[cfg(feature = "memory-storage")]
pub use self::memory::StorageOptions;

#[cfg(feature = "rocksdb-storage")]
mod rocksdb;
#[cfg(feature = "rocksdb-storage")]
pub use self::rocksdb::RocksDBStorage as Storage;
#[cfg(feature = "rocksdb-storage")]
pub use self::rocksdb::StorageOptions;

pub trait StorageBackend {
    fn enqueue(&self, id: Identifier, value: Value) -> Result<()>;
    fn dequeue(&self, id: Identifier) -> Result<Value>;
    fn length(&self, id: Identifier) -> Result<usize>;
    fn peek(&self, id: Identifier) -> Result<Value>;
}

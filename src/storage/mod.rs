use anyhow::Result;

use crate::types::*;

#[cfg(feature = "memory-storage")]
mod memory;
#[cfg(feature = "memory-storage")]
pub use self::memory::StorageOptions;
#[cfg(feature = "memory-storage")]
pub use memory::MemoryStorage as Storage;

#[cfg(feature = "rocksdb-storage")]
mod rocksdb;
#[cfg(feature = "rocksdb-storage")]
pub use self::rocksdb::RocksDBStorage as Storage;
#[cfg(feature = "rocksdb-storage")]
pub use self::rocksdb::StorageOptions;

#[async_trait::async_trait]
pub trait StorageBackend {
    async fn enqueue(&self, id: &Identifier, value: Value) -> Result<()>;
    async fn dequeue(&self, id: &Identifier) -> Result<Value>;
    async fn length(&self, id: &Identifier) -> Result<usize>;
    async fn peek(&self, id: &Identifier) -> Result<Value>;
}

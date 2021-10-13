use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};
use rocksdb::{ColumnFamilyDescriptor, Options, WriteBatch, DB};
use structopt::StructOpt;

use crate::errors::*;
use crate::storage::StorageBackend;
use crate::types::*;

#[derive(Debug, Clone, StructOpt)]
pub struct StorageOptions {
    #[structopt(short = "d", long = "database-path")]
    pub database_path: String,
}

#[derive(Debug, Clone)]
pub struct RocksDBStorage {
    db: Arc<RwLock<DB>>,
}

impl RocksDBStorage {
    #[tracing::instrument]
    pub fn init(path: &str) -> Result<Self> {
        Ok(Self {
            db: Arc::new(RwLock::new(
                DB::open_default(path).map_err(|_| StorageError::FailedInitialize)?,
            )),
        })
    }

    fn bound_keys(&self, id: &Identifier) -> (String, String) {
        (format!("{}:begin", &id.0), format!("{}:end", &id.0))
    }
}

#[async_trait::async_trait]
impl StorageBackend for RocksDBStorage {
    #[tracing::instrument]
    async fn enqueue(&self, id: Identifier, value: Value) -> Result<()> {
        let db = self.db.write().map_err(|_| StorageError::FailedLock)?;

        let (begin_key, end_key) = self.bound_keys(&id);
        let mut batch = WriteBatch::default();

        match db.get(&end_key)? {
            Some(end) => {
                let next = bincode::deserialize::<u64>(&end)? + 1;

                batch.put(&end_key, bincode::serialize(&next)?);
                batch.put(&format!("{}:{}", &id.0, next), bincode::serialize(&value)?);
            }
            None => {
                batch.put(&begin_key, bincode::serialize(&0)?);
                batch.put(&end_key, bincode::serialize(&1)?);
                batch.put(&format!("{}:{}", &id.0, 0), bincode::serialize(&value)?);
            }
        }

        db.write(batch)?;

        Ok(())
    }

    #[tracing::instrument]
    async fn dequeue(&self, id: Identifier) -> Result<Value> {
        let db = self.db.write().map_err(|_| StorageError::FailedLock)?;

        let (begin_key, _) = self.bound_keys(&id);

        let begin_data = db
            .get(&begin_key)?
            .ok_or(anyhow!(DataError::EmptyQueue(id.0.clone())))?;
        let begin = bincode::deserialize::<u64>(&begin_data)?;
        let next = begin + 1;

        let data = db
            .get(&format!("{}:{}", &id.0, begin))?
            .ok_or(anyhow!(DataError::EmptyQueue(id.0.clone())))?;

        db.put(&begin_key, bincode::serialize(&next)?)?;

        Ok(bincode::deserialize::<Value>(&data)?)
    }

    #[tracing::instrument]
    async fn length(&self, id: Identifier) -> Result<usize> {
        let db = self.db.read().map_err(|_| StorageError::FailedLock)?;

        let (begin_key, _) = self.bound_keys(&id);
        let end_key = format!("{}:end", &id.0);

        match db.get(&begin_key)? {
            Some(begin_data) => {
                let begin = bincode::deserialize::<u64>(&begin_data)?;
                let end_data = db
                    .get(&end_key)?
                    .ok_or(anyhow!(DataError::EmptyQueue(id.0)))?;
                let end = bincode::deserialize::<u64>(&end_data)?;

                Ok((end - begin) as usize)
            }
            None => Ok(0),
        }
    }

    #[tracing::instrument]
    async fn peek(&self, id: Identifier) -> Result<Value> {
        let db = self.db.read().map_err(|_| StorageError::FailedLock)?;

        let (begin_key, _) = self.bound_keys(&id);

        let begin_data = db
            .get(&begin_key)?
            .ok_or(anyhow!(DataError::EmptyQueue(id.clone().0)))?;
        let begin = bincode::deserialize::<u64>(&begin_data)?;
        let data = db
            .get(&format!("{}:{}", &id.0, begin))?
            .ok_or(anyhow!(DataError::EmptyQueue(id.0)))?;

        Ok(bincode::deserialize::<Value>(&data)?)
    }
}

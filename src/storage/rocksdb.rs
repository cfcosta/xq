use std::sync::Arc;

use anyhow::{anyhow, Result};
use rocksdb::{WriteBatch, DB};
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
    db: Arc<DB>,
}

impl RocksDBStorage {
    #[tracing::instrument]
    pub fn init(path: &str) -> Result<Self> {
        Ok(Self {
            db: Arc::new(
                DB::open_default(path).map_err(|_| StorageError::FailedInitialize)?,
            ),
        })
    }

    fn bound_keys(&self, id: &Identifier) -> (String, String) {
        (format!("{}:begin", &id.0), format!("{}:end", &id.0))
    }

    fn bounds_for(&self, id: &Identifier) -> Result<(u64, u64)> {
        let db = self.db.clone();

        let (begin_key, end_key) = self.bound_keys(id);

        let begin = db.get(&begin_key)?.ok_or(anyhow!(DataError::EmptyQueue(id.to_string())))?;
        let end = db.get(&end_key)?.ok_or(anyhow!(DataError::EmptyQueue(id.to_string())))?;

        Ok((serde_json::from_slice(&begin)?, serde_json::from_slice(&end)?))
    }
}

#[async_trait::async_trait]
impl StorageBackend for RocksDBStorage {
    #[tracing::instrument]
    async fn enqueue(&self, id: Identifier, value: Value) -> Result<()> {
        let db = self.db.clone();

        let (begin_key, end_key) = self.bound_keys(&id);
        let mut batch = WriteBatch::default();

        match self.bounds_for(&id) {
            Ok((_, end)) => {
                batch.put(&end_key, serde_json::to_vec(&(end + 1))?);
                batch.put(&format!("{}:{}", &id.0, &(end + 1)), serde_json::to_vec(&value)?);
            }
            _ => {
                batch.put(&begin_key, serde_json::to_vec(&0)?);
                batch.put(&end_key, serde_json::to_vec(&0)?);
                batch.put(&format!("{}:{}", &id.0, 0), serde_json::to_vec(&value)?);
            }
        }

        db.write(batch)?;

        Ok(())
    }

    #[tracing::instrument]
    async fn dequeue(&self, id: Identifier) -> Result<Value> {
        let db = self.db.clone();

        let (begin_key, _) = self.bound_keys(&id);
        let (begin, _) = self.bounds_for(&id)?;

        let data = db
            .get(&format!("{}:{}", &id.0, begin))?
            .ok_or(anyhow!(DataError::EmptyQueue(id.0.clone())))?;

        db.put(&begin_key, serde_json::to_vec(&(begin + 1))?)?;

        Ok(serde_json::from_slice::<Value>(&data)?)

    }

    #[tracing::instrument]
    async fn length(&self, id: Identifier) -> Result<usize> {
        match self.bounds_for(&id) {
            Ok((begin, end)) => Ok((end - begin + 1) as usize),
            _ => Ok(0)
        }
    }

    #[tracing::instrument]
    async fn peek(&self, id: Identifier) -> Result<Value> {
        let db = self.db.clone();

        let (begin, _) = self.bounds_for(&id)?;
        let data = db
            .get(&format!("{}:{}", &id.0, begin))?
            .ok_or(anyhow!(DataError::EmptyQueue(id.0)))?;

        Ok(serde_json::from_slice::<Value>(&data)?)
    }
}

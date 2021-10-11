use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use rocksdb::{ColumnFamilyDescriptor, Options, WriteBatch, DB};
use structopt::StructOpt;

use crate::errors::*;
use crate::types::*;
use crate::storage::StorageBackend;

#[derive(Debug, Clone, StructOpt)]
pub struct StorageOptions {
    #[structopt(short = "d", long = "database-path")]
    pub database_path: String
}

#[derive(Debug, Clone)]
pub struct RocksDBStorage {
    db: Arc<DB>,
}

impl RocksDBStorage {
    pub fn init(path: &str) -> Result<Self> {
        Ok(Self {
            db: Arc::new(
                DB::open_cf_descriptors(&Self::default_options(), path, vec![Self::data_cf()])
                    .map_err(|_| StorageError::FailedInitialize)?,
            ),
        })
    }

    fn default_options() -> Options {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts
    }

    fn data_cf() -> ColumnFamilyDescriptor {
        ColumnFamilyDescriptor::new("data", Self::default_options())
    }

    pub fn cleanup(path: &str) -> Result<()> {
        DB::destroy(&Options::default(), path)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl StorageBackend for RocksDBStorage {
    async fn enqueue(&self, id: Identifier, value: Value) -> Result<()> {
        let begin_key = format!("{}:begin", &id.0);
        let end_key = format!("{}:end", &id.0);
        let mut batch = WriteBatch::default();

        let cf = self.db.cf_handle("data").ok_or(anyhow!(StorageError::FailedInitialize))?;

        match self.db.get_cf(&cf, &end_key)? {
            Some(end) => {
                let next = bincode::deserialize::<u64>(&end)? + 1;

                batch.put_cf(&cf, &end_key, bincode::serialize(&next)?);
                batch.put_cf(&cf, &format!("{}:{}", &id.0, next), bincode::serialize(&value)?);
            }
            None => {
                batch.put_cf(&cf, &begin_key, bincode::serialize(&0)?);
                batch.put_cf(&cf, &end_key, bincode::serialize(&0)?);
                batch.put_cf(&cf, &format!("{}:{}", &id.0, 0), bincode::serialize(&value)?);
            }
        }

        self.db.write(batch)?;

        Ok(())
    }

    async fn dequeue(&self, id: Identifier) -> Result<Value> {
        let begin_key = format!("{}:begin", &id.0);
        let cf = self.db.cf_handle("data").ok_or(anyhow!(StorageError::FailedInitialize))?;

        match self.db.get_cf(&cf, &begin_key)? {
            Some(begin_data) => {
                let begin = bincode::deserialize::<u64>(&begin_data)?;
                let next = begin + 1;

                let data = self
                    .db
                    .get_cf(&cf, &format!("{}:{}", &id.0, begin))?
                    .ok_or(anyhow!(DataError::EmptyQueue(id.0)))?;

                self.db.put_cf(&cf, &begin_key, bincode::serialize(&next)?)?;

                Ok(bincode::deserialize::<Value>(&data)?)
            }
            None => {
                bail!(DataError::EmptyQueue(id.0))
            }
        }
    }

    async fn length(&self, id: Identifier) -> Result<usize> {
        let db = self.db.clone();
        let begin_key = format!("{}:begin", &id.0);
        let end_key = format!("{}:end", &id.0);
        let cf = self.db.cf_handle("data").ok_or(anyhow!(StorageError::FailedInitialize))?;

        match self.db.get_cf(&cf, &begin_key)? {
            Some(begin_data) => {
                let begin = bincode::deserialize::<u64>(&begin_data)?;
                let end_data = db
                    .get_cf(&cf, &end_key)?
                    .ok_or(anyhow!(DataError::EmptyQueue(id.0)))?;
                let end = bincode::deserialize::<u64>(&end_data)?;

                Ok((end - begin) as usize)
            }
            None => Ok(0),
        }
    }

    async fn peek(&self, id: Identifier) -> Result<Value> {
        let db = self.db.clone();
        let begin_key = format!("{}:begin", &id.0);
        let cf = self.db.cf_handle("data").ok_or(anyhow!(StorageError::FailedInitialize))?;

        let begin_data = db
            .get_cf(&cf, &begin_key)?
            .ok_or(anyhow!(DataError::EmptyQueue(id.clone().0)))?;
        let begin = bincode::deserialize::<u64>(&begin_data)?;
        let data = db
            .get_cf(&cf, &format!("{}:{}", &id.0, begin))?
            .ok_or(anyhow!(DataError::EmptyQueue(id.0)))?;

        Ok(bincode::deserialize::<Value>(&data)?)
    }
}

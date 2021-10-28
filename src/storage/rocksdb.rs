use std::{ sync::Arc, collections::VecDeque };

use anyhow::{Result, bail};
use rocksdb::{DB, MergeOperands, Options};
use structopt::StructOpt;
use serde::{Serialize, Deserialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Enqueue(Value),
    Dequeue
}

pub fn merge_queue(_new_key: &[u8], existing_val: Option<&[u8]>, operands: &mut MergeOperands) -> Option<Vec<u8>> {
    let mut current: VecDeque<Value> = match existing_val {
        Some(val) => bincode::deserialize::<VecDeque<Value>>(val).unwrap(),
        None => VecDeque::new()
    };

    for op in operands {
        match bincode::deserialize::<Operation>(op).unwrap() {
            Operation::Enqueue(v) => {
                current.push_back(v);
            },
            Operation::Dequeue => {
                current.pop_front();
            }
        }
    }

    Some(bincode::serialize(&current).unwrap())
}

impl RocksDBStorage {
    #[tracing::instrument]
    pub fn init(path: &str) -> Result<Self> {
        Ok(Self {
            db: Arc::new(DB::open(&Self::default_options(), path).map_err(|_| StorageError::FailedInitialize)?),
        })
    }

    fn default_options() -> Options {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_merge_operator_associative("queue merge operator", merge_queue);
        opts
    }
}

#[async_trait::async_trait]
impl StorageBackend for RocksDBStorage {
    #[tracing::instrument]
    async fn enqueue(&self, id: &Identifier, value: Value) -> Result<()> {
        let op = bincode::serialize(&Operation::Enqueue(value))?;
        self.db.merge(&id.0, op)?;

        Ok(())
    }

    #[tracing::instrument]
    async fn dequeue(&self, id: &Identifier) -> Result<Value> {
        let val = self.peek(id).await?;
        let op = bincode::serialize(&Operation::Dequeue)?;

        self.db.merge(&id.0, op)?;

        Ok(val)
    }

    #[tracing::instrument]
    async fn length(&self, id: &Identifier) -> Result<usize> {
        let db = self.db.clone();

        match db.get(&id.0)? {
            Some(v) => Ok(bincode::deserialize::<Vec<Value>>(&v)?.len()),
            None => Ok(0)
        }
    }

    #[tracing::instrument]
    async fn peek(&self, id: &Identifier) -> Result<Value> {
        match self.db.get(&id.0)? {
            Some(v) => {
                let value = bincode::deserialize::<Vec<Value>>(&v)?;
                match value.first() {
                    Some(v) => Ok(v.clone()),
                    None => bail!(DataError::EmptyQueue(id.0.clone()))
                }
            }
            None => bail!(DataError::EmptyQueue(id.0.clone()))
        }
    }
}

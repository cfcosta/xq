use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use rocksdb::{Options, WriteBatch, DB};

use crate::errors::*;
use crate::types::*;

pub trait Storage {
    fn enqueue(&mut self, id: Identifier, value: Value) -> Result<()>;
    fn dequeue(&mut self, id: Identifier) -> Result<Value>;
    fn length(&self, id: Identifier) -> Result<usize>;
    fn peek(&self, id: Identifier) -> Result<Value>;
}

pub struct DbStorage {
    db: Arc<DB>,
}

impl DbStorage {
    pub fn init(path: &str) -> Result<Self> {
        Ok(Self {
            db: Arc::new(DB::open_default(path).map_err(|_| DatabaseError::FailedInitialize)?),
        })
    }

    pub fn cleanup(path: &str) -> Result<()> {
        DB::destroy(&Options::default(), path)?;
        Ok(())
    }
}

impl Storage for DbStorage {
    fn enqueue(&mut self, id: Identifier, value: Value) -> Result<()> {
        let begin_key = format!("{}:begin", &id.0);
        let end_key = format!("{}:end", &id.0);
        let mut batch = WriteBatch::default();

        match self.db.get(&end_key)? {
            Some(end) => {
                let next = bincode::deserialize::<u64>(&end)? + 1;

                batch.put(&end_key, bincode::serialize(&next)?);
                batch.put(&format!("{}:{}", &id.0, next), bincode::serialize(&value)?);
            }
            None => {
                batch.put(&begin_key, bincode::serialize(&0)?);
                batch.put(&end_key, bincode::serialize(&0)?);
                batch.put(&format!("{}:{}", &id.0, 0), bincode::serialize(&value)?);
            }
        }

        self.db.write(batch)?;

        Ok(())
    }

    fn dequeue(&mut self, id: Identifier) -> Result<Value> {
        let begin_key = format!("{}:begin", &id.0);

        match self.db.get(&begin_key)? {
            Some(begin_data) => {
                let begin = bincode::deserialize::<u64>(&begin_data)?;
                let next = begin + 1;

                let data = self
                    .db
                    .get(&format!("{}:{}", &id.0, begin))?
                    .ok_or(anyhow!(DataError::EmptyQueue(id.0)))?;

                self.db.put(&begin_key, bincode::serialize(&next)?)?;

                Ok(bincode::deserialize::<Value>(&data)?)
            }
            None => {
                bail!(DataError::EmptyQueue(id.0))
            }
        }
    }

    fn length(&self, id: Identifier) -> Result<usize> {
        let db = self.db.clone();
        let begin_key = format!("{}:begin", &id.0);
        let end_key = format!("{}:end", &id.0);

        match self.db.get(&begin_key)? {
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

    fn peek(&self, id: Identifier) -> Result<Value> {
        let db = self.db.clone();
        let begin_key = format!("{}:begin", &id.0);

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

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, bail, Result};
use structopt::StructOpt;

use crate::errors::*;
use crate::storage::StorageBackend;
use crate::types::*;

#[derive(Debug, Clone, StructOpt)]
pub struct StorageOptions {}

#[derive(Debug, Clone)]
pub struct MemoryStorage {
    map: Arc<RwLock<HashMap<Identifier, VecDeque<Value>>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl StorageBackend for MemoryStorage {
    fn enqueue(&self, id: Identifier, value: Value) -> Result<()> {
        let mut map = self.map.write().map_err(|_| StorageError::FailedLock)?;

        match map.get_mut(&id) {
            Some(v) => {
                v.push_back(value);
            }
            None => {
                let mut deque = VecDeque::new();
                deque.push_back(value);

                map.insert(id, deque);
            }
        }
        Ok(())
    }

    fn dequeue(&self, id: Identifier) -> Result<Value> {
        let mut map = self.map.write().map_err(|_| StorageError::FailedLock)?;

        match map.get_mut(&id) {
            Some(q) => match q.pop_front() {
                Some(v) => Ok(v),
                None => bail!(DataError::EmptyQueue(id.0)),
            },
            None => bail!(DataError::EmptyQueue(id.0)),
        }
    }

    fn length(&self, id: Identifier) -> Result<usize> {
        let map = self.map.read().map_err(|_| StorageError::FailedLock)?;

        Ok(map.get(&id).map(|x| x.len()).unwrap_or(0))
    }

    fn peek(&self, id: Identifier) -> Result<Value> {
        let map = self.map.read().map_err(|_| StorageError::FailedLock)?;

        Ok(map
            .get(&id)
            .ok_or(anyhow!(DataError::EmptyQueue(id.clone().0)))?
            .front()
            .ok_or(anyhow!(DataError::EmptyQueue(id.0)))?
            .clone())
    }
}

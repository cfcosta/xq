use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, bail, Result};
use structopt::StructOpt;

use crate::errors::*;
use crate::storage::StorageBackend;
use crate::types::*;

#[derive(Debug, Clone, StructOpt)]
pub struct StorageOptions {}

#[derive(Debug)]
pub struct MemoryStorage {
    map: HashMap<Identifier, VecDeque<Value>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl StorageBackend for MemoryStorage {
    fn enqueue(&mut self, id: Identifier, value: Value) -> Result<()> {
        match self.map.get_mut(&id) {
            Some(v) => {
                v.push_back(value);
            }
            None => {
                let mut deque = VecDeque::new();
                deque.push_back(value);

                self.map.insert(id, deque);
            }
        }
        Ok(())
    }

    fn dequeue(&mut self, id: Identifier) -> Result<Value> {
        match self.map.get_mut(&id) {
            Some(q) => match q.pop_front() {
                Some(v) => Ok(v),
                None => bail!(DataError::EmptyQueue(id.0)),
            },
            None => bail!(DataError::EmptyQueue(id.0)),
        }
    }

    fn length(&self, id: Identifier) -> Result<usize> {
        Ok(self.map.get(&id).map(|x| x.len()).unwrap_or(0))
    }

    fn peek(&self, id: Identifier) -> Result<Value> {
        Ok(self
            .map
            .get(&id)
            .ok_or(anyhow!(DataError::EmptyQueue(id.clone().0)))?
            .front()
            .ok_or(anyhow!(DataError::EmptyQueue(id.0)))?
            .clone())
    }
}

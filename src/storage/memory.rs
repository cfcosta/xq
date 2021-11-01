use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

use anyhow::Result;
use structopt::StructOpt;

use crate::errors::*;
use crate::storage::StorageBackend;
use crate::types::*;

#[derive(Debug, Clone, StructOpt)]
pub struct StorageOptions {}

#[derive(Debug, Clone)]
pub struct MemoryStorage {
    map: Arc<RwLock<HashMap<Identifier, Item>>>,
}

#[derive(Debug)]
pub struct Item {
    bounds: (usize, usize),
    data: Vec<Value>
}

impl Default for Item {
    fn default() -> Self {
        Self {
            bounds: (0, 0),
            data: Default::default()
        }
    }
}

impl Item {
    #[inline(always)]
    fn enqueue(&mut self, v: Value) {
        let (start, end) = self.bounds;
        self.bounds = (start, end + 1);

        self.data.push(v);
    }

    #[inline(always)]
    fn dequeue(&mut self) -> Option<&mut Value> {
        let (start, end) = self.bounds;

        self.bounds = (start + 1, end);
        self.data.get_mut(self.bounds.0)
    }

    #[inline(always)]
    fn peek(&self) -> Option<&Value> {
        self.data.get(self.bounds.0)
    }

    #[inline(always)]
    fn length(&self) -> usize {
        let (start, end) = self.bounds;
        end - start
    }
}

impl MemoryStorage {
    #[tracing::instrument]
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl StorageBackend for MemoryStorage {
    #[tracing::instrument]
    fn enqueue(&self, id: &Identifier, value: Value) -> Result<()> {
        let mut map = self.map.write().map_err(|_| StorageError::FailedLock)?;

        match map.get_mut(&id) {
            Some(v) => v.enqueue(value),
            None => {
                let mut item = Item::default();
                item.enqueue(value);

                map.insert(id.clone(), item);
            }
        }
        Ok(())
    }

    #[tracing::instrument]
    fn dequeue(&self, id: &Identifier) -> Result<Value> {
        let mut map = self.map.write().map_err(|_| StorageError::FailedLock)?;

        match map.get_mut(&id) {
            Some(q) => match q.dequeue() {
                Some(v) => Ok(v.clone()),
                None => Ok(Value::Null),
            },
            None => Ok(Value::Null),
        }
    }

    #[tracing::instrument]
    fn length(&self, id: &Identifier) -> Result<usize> {
        let map = self.map.read().map_err(|_| StorageError::FailedLock)?;

        Ok(map.get(&id).map(|x| x.length()).unwrap_or(0))
    }

    #[tracing::instrument]
    fn peek(&self, id: &Identifier) -> Result<Value> {
        let map = self.map.read().map_err(|_| StorageError::FailedLock)?;

        match map.get(&id).and_then(|x| x.peek()) {
            Some(v) => Ok(v.clone()),
            None => Ok(Value::Null),
        }
    }
}

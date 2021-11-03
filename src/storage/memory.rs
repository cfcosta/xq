use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use anyhow::{bail, Result};
use structopt::StructOpt;

use crate::errors::*;
use crate::storage::StorageBackend;
use crate::types::*;

#[derive(Debug, Clone, StructOpt)]
pub struct StorageOptions {}

#[derive(Debug, Clone)]
pub struct MemoryStorage {
    map: Arc<RwLock<BTreeMap<Identifier, Item>>>,
}

#[derive(Debug)]
pub struct Item {
    kind: QueueType,
    bounds: (usize, usize),
    data: Vec<Value>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            kind: QueueType::Integer,
            bounds: (0, 0),
            data: Default::default(),
        }
    }
}

impl Item {
    #[inline(always)]
    fn enqueue(&mut self, v: Value) -> Result<()> {
        if v.kind() == self.kind {
            let (start, end) = self.bounds;
            self.bounds = (start, end + 1);

            self.data.push(v);
            Ok(())
        } else {
            bail!(DataError::IncorrectType {
                expected: v.kind(),
                got: self.kind
            })
        }
    }

    #[inline(always)]
    fn dequeue(&mut self) -> Option<&mut Value> {
        let (start, end) = self.bounds;

        self.bounds = (start + 1, end);
        self.data.get_mut(start)
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

#[test]
fn enqueued_item_is_dequeued_correctly() -> Result<()> {
    let mut item = Item::default();
    item.enqueue(Value::Integer(1))?;
    assert_eq!(item.dequeue(), Some(&mut Value::Integer(1)));

    Ok(())
}

impl MemoryStorage {
    #[tracing::instrument]
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(Default::default())),
        }
    }
}

#[async_trait::async_trait]
impl StorageBackend for MemoryStorage {
    #[tracing::instrument]
    fn open(&self, id: &Identifier, kind: QueueType) -> Result<()> {
        let mut map = self.map.write().map_err(|_| StorageError::FailedLock)?;

        match map.get(id) {
            Some(_) => bail!(DataError::AlreadyOpen {
                queue: id.0.clone()
            }),
            None => {
                map.insert(
                    id.clone(),
                    Item {
                        kind,
                        ..Default::default()
                    },
                );

                Ok(())
            }
        }
    }

    fn close(&self, id: &Identifier) -> Result<()> {
        let mut map = self.map.write().map_err(|_| StorageError::FailedLock)?;

        match map.get(&id) {
            Some(_) => {
                map.remove(id);
                Ok(())
            }
            None => bail!(DataError::ClosedQueue {
                queue: id.0.clone()
            }),
        }
    }

    #[tracing::instrument]
    fn enqueue(&self, id: &Identifier, value: Value) -> Result<()> {
        let mut map = self.map.write().map_err(|_| StorageError::FailedLock)?;

        match map.get_mut(&id) {
            Some(v) => {
                v.enqueue(value)?;
                Ok(())
            }
            None => bail!(DataError::ClosedQueue {
                queue: id.0.clone()
            }),
        }
    }

    #[tracing::instrument]
    fn dequeue(&self, id: &Identifier) -> Result<Value> {
        let mut map = self.map.write().map_err(|_| StorageError::FailedLock)?;

        match map.get_mut(&id) {
            Some(q) => match q.dequeue() {
                Some(v) => Ok(v.clone()),
                None => Ok(Value::Null),
            },
            None => bail!(DataError::ClosedQueue {
                queue: id.0.clone()
            }),
        }
    }

    #[tracing::instrument]
    fn length(&self, id: &Identifier) -> Result<usize> {
        let map = self.map.read().map_err(|_| StorageError::FailedLock)?;

        match map.get(&id) {
            Some(v) => Ok(v.length()),
            None => bail!(DataError::ClosedQueue {
                queue: id.0.clone()
            }),
        }
    }

    #[tracing::instrument]
    fn peek(&self, id: &Identifier) -> Result<Value> {
        let map = self.map.read().map_err(|_| StorageError::FailedLock)?;

        match map.get(&id) {
            Some(item) => Ok(item.peek().unwrap_or(&Value::Null).clone()),
            None => bail!(DataError::ClosedQueue {
                queue: id.0.clone()
            }),
        }
    }
}

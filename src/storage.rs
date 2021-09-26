use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Result};

use crate::types::*;
use crate::errors::*;

pub trait Storage {
    fn enqueue(&mut self, id: Identifier, value: Value) -> Result<()>;
    fn dequeue(&mut self, id: Identifier) -> Result<Value>;
}

#[derive(Debug)]
pub struct MemStore {
    map: HashMap<Identifier, VecDeque<Value>>,
}

impl MemStore {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl Storage for MemStore {
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
}

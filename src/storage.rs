use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Result};

use crate::command::*;
use crate::errors::*;

trait Storage {
    fn enqueue(&mut self, id: Identifier, value: Value) -> Result<()>;
    fn dequeue(&mut self, id: Identifier) -> Result<Value>;
}

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
        self.map
            .entry(id)
            .and_modify(|entry| entry.push_back(value))
            .or_insert(VecDeque::new());
        Ok(())
    }

    fn dequeue(&mut self, id: Identifier) -> Result<Value> {
        let deque = self.map.get_mut(&id);

        match deque {
            Some(q) => match q.pop_front() {
                Some(v) => Ok(v),
                None => bail!(DataError::EmptyQueue(id.0)),
            },
            None => bail!(DataError::EmptyQueue(id.0)),
        }
    }
}

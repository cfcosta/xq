use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Result, anyhow};

use crate::errors::*;
use crate::types::*;

pub trait Storage {
    fn enqueue(&mut self, id: Identifier, value: Value) -> Result<()>;
    fn dequeue(&mut self, id: Identifier) -> Result<Value>;
    fn length(&self, id: Identifier) -> Result<usize>;
    fn peek(&self, id: Identifier) -> Result<&Value>;
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

    fn length(&self, id: Identifier) -> Result<usize> {
        Ok(self.map.get(&id).map(|x| x.len()).unwrap_or(0))
    }

    fn peek(&self, id: Identifier) -> Result<&Value> {
        self.map
            .get(&id)
            .ok_or(anyhow!(DataError::EmptyQueue(id.clone().0)))?
            .front()
            .ok_or(anyhow!(DataError::EmptyQueue(id.0)))
    }
}

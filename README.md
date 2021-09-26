# Xq

Please remember that this is not even alpha-quality software. Even if at some
point it does get to that level of production-readyness, it's scope is still
too small to be of any use to anyone.

Consider this a study/reference project for someone else that wants to do the
same thing.
---

Xq is a very simple distributed queueing system implemented in rust.

## Roadmap

- [ ] Parser
  - [x] Enqueue
  - [x] Dequeue
  - [ ] Length
  - [ ] Asserts
- [ ] Storage
  - [ ] Simple in-memory Storage
  - [ ] RocksDB based storage
- [ ] Networking
  - [ ] TCP/UDP Server/Daemon
  - [ ] Raft Consensus

## Syntax Reference

- Enqueue
  ```
  enqueue key 1
  enqueue key 1.12
  enqueue key 1.16e12
  enqueue key "string key"
  ```

- Dequeue
  ```
  dequeue key
  ```

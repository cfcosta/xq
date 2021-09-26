# Xq

Please remember that this is not even alpha-quality software. Even if at some
point it does get to that level of production-readyness, it's scope is still
too small to be of any use to anyone.

Consider this a study/reference project for someone else that wants to do the
same thing.

---

Xq is a very simple distributed queueing system implemented in rust. At some
point, it will be a fully distributed/replicated queueing system on top of
state-of-the art projects (namely the [Raft Consensus
Algorithm](https://raft.github.io/) and [RocksDB](https://rocksdb.org/)).

It has it's own language for commands (similar to Redis), using the [nom parser
combinator library](https://github.com/Geal/nom).

## Values

- Production-ready, not production-useful.
- Move slowly and maintain things.

## Roadmap

- [ ] Parser
  - [x] Enqueue
  - [x] Dequeue
  - [ ] Length
  - [ ] Assert
  - [ ] Raft-related calls
    - [ ] Append Entries
    - [ ] Request Vote
    - [ ] Install Snapshot
- [ ] Storage
  - [ ] Simple in-memory Storage
  - [ ] RocksDB based storage
  - [ ] Snapshotting
- [ ] Networking
  - [ ] TCP/UDP Server/Daemon
  - [ ] TCP/UDP Client
- [ ] Raft Consensus
  - [ ] RPC Calls (Networking)
  - [ ] Storage
- [ ] Benchmarking

## Possible Future Goals

- [ ] REPL
- [ ] Lua Scripting
- [ ] Language Server

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

# Xq

Please remember that this is not even alpha-quality software. Even if at some
point it does get to that level of production-readiness, it's scope is still
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
  - [x] Value Types
    - [x] Integers
    - [x] Floats
    - [x] Strings
    - [x] Null
  - [ ] Open
  - [ ] Close
  - [x] Enqueue
  - [x] Dequeue
  - [x] Length
  - [x] Peek
  - [x] Assert
  - [x] Assert error
  - [ ] Raft-related calls
    - [ ] Append Entries
    - [ ] Request Vote
    - [ ] Install Snapshot
  - [ ] Error Reporting
- [ ] Storage
  - [x] Simple in-memory Storage
  - [x] RocksDB based storage
  - [ ] Snapshotting
- [ ] Networking
  - [x] TCP Server/Daemon
  - [ ] TCP Client
    - [x] REPL
    - [ ] Readline
    - [ ] Syntax Highlithing
    - [ ] Error Reporting
- [ ] Raft Consensus
  - [ ] RPC Calls (Networking)
  - [ ] Storage
- [ ] Benchmarking

## Possible Future Goals

- [ ] REPL
- [ ] Lua Scripting
- [ ] Language Server

## Developing

Default Rust instructions apply here:

```sh
git clone git@github.com:cfcosta/xq
cd xq
cargo build
```

To run tests:

```sh
cargo test
```

The project also contains its own test runner that starts a new instance of the
storage, runs a sequence of commands in a file, and fails if there are any errors:

```sh
./test.sh
```

## Running

### Server 

To start the server:

```
cargo run --release --bin xqd
```

Server should be available at `localhost:8080`

### Client 

To connect to a server:

```
cargo run --release --bin xq -- 127.0.0.1:8080
```

## Security and Reliability Assumptions

- As with all applications that use Raft, we assume that the actors are
  well-intentioned and not compromised.
- The cluster should keep running reliably as long as at least 51% of the nodes
  are up.

## Syntax Reference

To start using a queue, you need to open it, along with this types type. The
available types are:

- `:integer`
- `:float`
- `:string`
- `:null`

Closing a queue stops it from receiving values, as well as removing the current
ones on the queue. There isn't any performance cost of keeping the keys there
after they've been persisted to the WAL log, so it isn't necessary.

### Open

```
open a :integer
open b :float
open c :string
open d :null
```

### Close

```
close a
```

### Enqueue

Adds a value to a queue. If the queue does not exist, create it.

```
enqueue key 1
enqueue key 1.12
enqueue key 1.16e12
enqueue key "string key"
```

### Dequeue

Removes a value from a queue. If the queue is empty or not initialized, returns null.

```
dequeue key
```

### Peek

Returns the head of the queue without removing it

```
peek key
```

### Length

Returns the length of a current queue. Returns 0 if the queue is not initialized.

```
length key
```

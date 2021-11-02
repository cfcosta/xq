use std::fmt::Debug;

use anyhow::Result;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mktemp::Temp;

use xq::{
    parser, run_command,
    storage::{Storage, StorageBackend},
    types::*,
};

fn setup_storage() -> Result<impl StorageBackend + Debug + Sync> {
    let path = Temp::new_dir()?.to_path_buf().display().to_string();

    #[cfg(feature = "memory-storage")]
    return Ok(Storage::new());

    #[cfg(feature = "rocksdb-storage")]
    return Ok(Storage::init(&path)?);
}

fn criterion_benchmark(c: &mut Criterion) -> Result<()> {
    c.bench_function("parsing", |b| {
        b.iter(|| {
            assert_eq!(
                parser::parse("enqueue a 1").unwrap(),
                black_box(vec![Command::enqueue("a", 1)])
            );
        })
    });

    let storage = setup_storage().unwrap();
    c.bench_function("enqueue", |b| {
        b.iter(|| run_command(&storage, black_box(Command::enqueue("a", 1))).unwrap())
    });

    let storage = setup_storage().unwrap();
    run_command(&storage, black_box(Command::enqueue("multipeeks", 1))).unwrap();
    c.bench_function("multiple peeks", |b| {
        b.iter(|| {
            run_command(&storage, black_box(Command::peek("multipeeks"))).unwrap();
        });
    });

    let storage = setup_storage().unwrap();
    c.bench_function("enqueue + dequeue", |b| {
        run_command(&storage, black_box(Command::enqueue("b", 1))).unwrap();
        b.iter(|| {
            run_command(&storage, black_box(Command::dequeue("b"))).unwrap();
        })
    });

    let storage = setup_storage().unwrap();
    for _ in 1..100 {
        run_command(&storage, black_box(Command::enqueue("b", 1))).unwrap();
    }

    c.bench_function("enqueue * 100 + dequeue", |b| {
        b.iter(|| {
            run_command(&storage, black_box(Command::dequeue("b"))).unwrap();
        })
    });

    let storage = setup_storage().unwrap();
    for _ in 1..1000 {
        run_command(&storage, black_box(Command::enqueue("c", 1))).unwrap();
    }
    c.bench_function("enqueue * 1000 + dequeue", |b| {
        b.iter(|| {
            run_command(&storage, black_box(Command::dequeue("c"))).unwrap();
        })
    });

    Ok(())
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

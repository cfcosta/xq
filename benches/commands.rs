use anyhow::Result;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mktemp::Temp;

use xq::{parser, run_command, storage::Storage, types::*};

fn criterion_benchmark(c: &mut Criterion) -> Result<()> {
    let path = Temp::new_dir()?.to_path_buf().display().to_string();

    #[cfg(feature = "memory-storage")]
    let storage = Storage::new();
    #[cfg(feature = "rocksdb-storage")]
    let storage = Storage::init(&path)?;

    c.bench_function("parsing", |b| {
        b.iter(|| {
            assert_eq!(
                parser::parse("enqueue a 1").unwrap(),
                vec![Command::enqueue("a", 1)]
            );
        })
    });

    c.bench_function("enqueue", |b| {
        b.iter(|| run_command(&storage, black_box(Command::enqueue("a", 1))).unwrap())
    });

    c.bench_function("multiple peeks", |b| {
        b.iter(|| {
            run_command(&storage, black_box(Command::enqueue("multipeeks", 1))).unwrap();

            for _ in 1..100 {
                run_command(&storage, black_box(Command::peek("multipeeks"))).unwrap();
            }
        });
    });

    c.bench_function("enqueue + dequeue", |b| {
        b.iter(|| {
            run_command(&storage, black_box(Command::enqueue("b", 1))).unwrap();
            run_command(&storage, black_box(Command::dequeue("b"))).unwrap();
        })
    });

    c.bench_function("enqueue * 100 + dequeue", |b| {
        b.iter(|| {
            for _ in 1..100 {
                run_command(&storage, black_box(Command::enqueue("b", 1))).unwrap();
            }

            run_command(&storage, black_box(Command::dequeue("b"))).unwrap();
        })
    });

    c.bench_function("enqueue * 1000 + dequeue", |b| {
        b.iter(|| {
            for _ in 1..1000 {
                run_command(&storage, black_box(Command::enqueue("b", 1))).unwrap();
            }

            run_command(&storage, black_box(Command::dequeue("b"))).unwrap();
        })
    });

    Ok(())
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

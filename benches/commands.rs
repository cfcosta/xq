use anyhow::Result;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mktemp::Temp;

use xq::{run_command, storage::Storage, types::*};

fn criterion_benchmark(c: &mut Criterion) -> Result<()> {
    let path = Temp::new_dir()?.to_path_buf().display().to_string();

    let rt = tokio::runtime::Runtime::new().unwrap();

    #[cfg(feature = "memory-storage")]
    let storage = Storage::new();
    #[cfg(feature = "rocksdb-storage")]
    let storage = Storage::init(&path)?;

    c.bench_function("enqueue", |b| {
        b.to_async(&rt).iter(|| async {
            run_command(
                &storage,
                black_box(Command::Enqueue(Identifier("a".into()), Value::Integer(1))),
            )
            .await
            .unwrap()
        })
    });

    c.bench_function("multiple peeks", |b| {
        b.to_async(&rt).iter(|| async {
            run_command(
                &storage,
                black_box(Command::Enqueue(
                    Identifier("multipeeks".into()),
                    Value::Integer(1),
                )),
            )
            .await
            .unwrap();

            for _ in 1..100 {
                run_command(
                    &storage,
                    black_box(Command::Peek(Identifier("multipeeks".into()))),
                )
                .await
                .unwrap();
            }
        });
    });

    c.bench_function("enqueue + dequeue", |b| {
        b.to_async(&rt).iter(|| async {
            run_command(
                &storage,
                black_box(Command::Enqueue(Identifier("b".into()), Value::Integer(1))),
            )
            .await
            .unwrap();
            run_command(
                &storage,
                black_box(Command::Dequeue(Identifier("b".into()))),
            )
            .await
            .unwrap();
        })
    });

    c.bench_function("enqueue * 100 + dequeue", |b| {
        b.to_async(&rt).iter(|| async {
            for _ in 1..100 {
                run_command(
                    &storage,
                    black_box(Command::Enqueue(Identifier("b".into()), Value::Integer(1))),
                )
                .await
                .unwrap();
            }
            run_command(
                &storage,
                black_box(Command::Dequeue(Identifier("b".into()))),
            )
            .await
            .unwrap();
        })
    });

    Ok(())
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

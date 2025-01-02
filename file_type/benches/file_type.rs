use criterion::{criterion_group, criterion_main, Criterion};
use file_type::{FileType, Result};
use std::path::PathBuf;
use tokio::runtime::Runtime;

const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn bench_lifecycle(criterion: &mut Criterion) -> Result<()> {
    let runtime = Runtime::new()?;
    let file = PathBuf::from(CRATE_DIR)
        .join("..")
        .join("testdata")
        .join("pronom")
        .join("x-fmt-263-signature-id-200.zip");

    criterion.bench_function("classify_file", |bencher| {
        bencher.iter(|| {
            runtime.block_on(async {
                classify_file(&file).await.ok();
            });
        });
    });

    Ok(())
}

async fn classify_file(class_file: &PathBuf) -> Result<()> {
    FileType::try_from_file(class_file).await?;
    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmarks
);
criterion_main!(benches);

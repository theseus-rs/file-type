use criterion::{criterion_group, criterion_main, Criterion};
use file_type::FileType;
use std::fs;
use std::path::PathBuf;

const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn benchmarks(criterion: &mut Criterion) {
    bench_lifecycle(criterion).ok();
}

fn large_bytes() -> Vec<u8> {
    let length = 1 << 31;
    let mut bytes = vec![0; length];
    bytes[0] = 0xFD;
    bytes[1] = 0x37;
    bytes[2] = 0x7A;
    bytes[3] = 0x58;
    bytes[4] = 0x5A;
    bytes[5] = 0x00;
    bytes[length - 2] = 0x59;
    bytes[length - 1] = 0x5A;
    bytes
}

fn bench_lifecycle(criterion: &mut Criterion) -> anyhow::Result<()> {
    let file = PathBuf::from(CRATE_DIR)
        .join("..")
        .join("testdata")
        .join("pronom")
        .join("x-fmt-263-signature-id-200.zip");
    let bytes = fs::read(&file)?;
    let large_bytes = large_bytes();

    criterion.bench_function("from_id", |bencher| {
        bencher.iter(|| {
            let _ = FileType::from_id("fmt/1");
        });
    });

    criterion.bench_function("from_extension", |bencher| {
        bencher.iter(|| {
            let _ = FileType::from_extension("parquet");
        });
    });

    criterion.bench_function("from_media_type", |bencher| {
        bencher.iter(|| {
            let _ = FileType::from_media_type("application/vnd.apache.parquet");
        });
    });

    criterion.bench_function("from_bytes", |bencher| {
        bencher.iter(|| {
            let _ = FileType::from_bytes(&bytes);
        });
    });

    criterion.bench_function("try_from_file", |bencher| {
        bencher.iter(|| {
            let _ = FileType::try_from_file_sync(&file);
        });
    });

    criterion.bench_function("from_bytes (2GiB)", |bencher| {
        bencher.iter(|| {
            let _ = FileType::from_bytes(&large_bytes);
        });
    });

    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmarks
);
criterion_main!(benches);

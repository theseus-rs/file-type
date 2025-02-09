use criterion::{criterion_group, criterion_main, Criterion};
use file_type::FileType;
use std::fs;
use std::path::PathBuf;

const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const PNG_EXTENSION: &str = "png";
const PNG_MEDIA_TYPE: &str = "image/png";

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
        .join("..")
        .join("..")
        .join("test_data")
        .join("pronom")
        .join("pronom-382-signature-0.zip");
    let bytes = fs::read(&file)?;
    let large_bytes = large_bytes();

    criterion.bench_function("file_type::from_extension", |bencher| {
        bencher.iter(|| {
            let _ = FileType::from_extension(PNG_EXTENSION);
        });
    });

    criterion.bench_function("file_type::from_media_type", |bencher| {
        bencher.iter(|| {
            let _ = FileType::from_media_type(PNG_MEDIA_TYPE);
        });
    });

    criterion.bench_function("file_type::from_bytes", |bencher| {
        bencher.iter(|| {
            let _ = FileType::from_bytes(&bytes);
        });
    });

    criterion.bench_function("file_type::try_from_file", |bencher| {
        bencher.iter(|| {
            let _ = FileType::try_from_file_sync(&file);
        });
    });

    criterion.bench_function("file_type::from_bytes (2GiB)", |bencher| {
        bencher.iter(|| {
            let _ = FileType::from_bytes(&large_bytes);
        });
    });

    //
    // Comparison testing to the file-format crate
    //

    criterion.bench_function("file-format::from_bytes", |bencher| {
        bencher.iter(|| {
            let _ = file_format::FileFormat::from_bytes(&bytes);
        });
    });

    criterion.bench_function("file-format::from_bytes (2GiB)", |bencher| {
        bencher.iter(|| {
            let _ = file_format::FileFormat::from_bytes(&large_bytes);
        });
    });

    //
    // Comparison testing to the infer crate
    //

    criterion.bench_function("infer::from_bytes", |bencher| {
        bencher.iter(|| {
            let _ = infer::get(&bytes);
        });
    });

    criterion.bench_function("infer::from_bytes (2GiB)", |bencher| {
        bencher.iter(|| {
            let _ = infer::get(&large_bytes);
        });
    });

    //
    // Comparison testing to the magic crate
    //
    #[cfg(target_os = "linux")]
    {
        let cookie = magic::Cookie::open(Default::default())?;
        let cookie = cookie
            .load(&Default::default())
            .expect("failed to load magic database");

        criterion.bench_function("magic::from_bytes", |bencher| {
            bencher.iter(|| {
                // human-readable description, more than a static name
                cookie.set_flags(magic::cookie::Flags::ERROR).unwrap();
                let _ = cookie.buffer(&bytes).unwrap();

                // file type extensions
                cookie
                    .set_flags(magic::cookie::Flags::ERROR | magic::cookie::Flags::EXTENSION)
                    .unwrap();
                let _ = cookie.buffer(&bytes).unwrap();

                // media type
                cookie
                    .set_flags(magic::cookie::Flags::ERROR | magic::cookie::Flags::MIME_TYPE)
                    .unwrap();
                let _ = cookie.buffer(&bytes).unwrap();
            });
        });
    }

    //
    // Comparison testing to the mime_guess crate
    //

    criterion.bench_function("mime_guess::from_extension", |bencher| {
        bencher.iter(|| {
            let _ = mime_guess::get_mime_extensions_str(PNG_EXTENSION);
        });
    });

    criterion.bench_function("mime_guess::from_media_type", |bencher| {
        bencher.iter(|| {
            let _ = mime_guess::get_mime_extensions_str(PNG_MEDIA_TYPE);
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

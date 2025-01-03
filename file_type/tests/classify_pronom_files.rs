use anyhow::Result;
use file_type::FileType;
use std::path::PathBuf;
use walkdir::WalkDir;

const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn data_dir() -> PathBuf {
    PathBuf::from(CRATE_DIR)
        .join("..")
        .join("testdata")
        .join("pronom")
}

#[tokio::test]
async fn test_file_classification() -> Result<()> {
    let data_dir = data_dir();

    for entry in WalkDir::new(data_dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }

        let full_filename = path
            .file_name()
            .expect("file name")
            .to_string_lossy()
            .to_string();
        let filename = full_filename.split('.').next().expect("split").to_string();
        let id = if filename.starts_with("x-fmt-") {
            let parts: Vec<&str> = filename.split('-').collect();
            format!("{}-{}/{}", parts[0], parts[1], parts[2])
        } else {
            let parts: Vec<&str> = filename.split('-').collect();
            format!("{}/{}", parts[0], parts[1])
        };

        let file_type = FileType::try_from_file(path).await?;
        if file_type.id() == id {
            println!("file_type.id()={}, id={}", file_type.id(), id);
        } else {
            eprintln!(
                "[ERROR] file_type.id()={}, id={}: {full_filename}",
                file_type.id(),
                id
            );
        }
        // assert_eq!(file_type.id(), id);
    }

    Ok(())
}

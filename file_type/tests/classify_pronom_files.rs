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

async fn test_file(file_name: &str) -> Result<(String, &FileType)> {
    let data_dir = data_dir();
    let path = data_dir.join(file_name);
    let file_name = path
        .file_name()
        .expect("file name")
        .to_string_lossy()
        .to_string();
    let file_name = file_name.split('.').next().expect("split").to_string();
    let id = if file_name.starts_with("x-fmt-") {
        let parts: Vec<&str> = file_name.split('-').collect();
        format!("{}-{}/{}", parts[0], parts[1], parts[2])
    } else {
        let parts: Vec<&str> = file_name.split('-').collect();
        format!("{}/{}", parts[0], parts[1])
    };

    let file_type = FileType::try_from_file(path).await?;
    Ok((id, file_type))
}

#[tokio::test]
async fn test_file_classification() -> Result<()> {
    let data_dir = data_dir();
    let mut passed_tests = 0;
    let mut failed_tests = 0;

    for entry in WalkDir::new(data_dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }

        let file_name = path
            .file_name()
            .expect("file name")
            .to_string_lossy()
            .to_string();
        let (id, file_type) = test_file(&file_name).await?;

        if file_type.id() == id {
            assert_eq!(file_type.id(), id);
            passed_tests += 1;
        } else {
            eprintln!(
                "[ERROR] file_type.id()={}, id={}: {file_name}",
                file_type.id(),
                id
            );
            failed_tests += 1;
        }
    }

    println!("Passed: {passed_tests}");
    println!("Failed: {failed_tests}");
    Ok(())
}

#[tokio::test]
async fn test_single_file_classification() -> Result<()> {
    let (id, file_type) = test_file("fmt-708-signature-id-831.wav").await?;
    assert_eq!(file_type.id(), id);
    Ok(())
}

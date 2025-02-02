use anyhow::Result;
use file_type::FileType;
use std::fs;
use std::path::PathBuf;

const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn data_dir() -> PathBuf {
    PathBuf::from(CRATE_DIR)
        .join("..")
        .join("testdata")
        .join("pronom")
}

fn test_file(file_name: &str) -> Result<(String, &FileType)> {
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

    let file_type = FileType::try_from_file_sync(path)?;
    Ok((id, file_type))
}

#[test]
fn test_file_classification() -> Result<()> {
    let data_dir = data_dir();
    let mut passed_tests = 0;
    let mut errored_tests = 0;

    for entry in fs::read_dir(data_dir)? {
        let path = entry?.path();
        if path.is_dir() {
            continue;
        }

        let file_name = path
            .file_name()
            .expect("file name")
            .to_string_lossy()
            .to_string();
        let (id, file_type) = test_file(&file_name)?;

        if file_type.id() == id {
            passed_tests += 1;
        } else {
            errored_tests += 1;
            eprintln!(
                "ERROR id={id}, file_type.id()={}: {file_name}",
                file_type.id(),
            );
        }
    }

    println!("Passed: {passed_tests}");
    println!("Errored: {errored_tests}");
    assert!(passed_tests > 1700);
    Ok(())
}

#[test]
fn test_single_file_classification() -> Result<()> {
    let (id, file_type) = test_file("pronom-1507-signature-id-831.wav")?;
    assert_eq!(file_type.id(), id);
    Ok(())
}

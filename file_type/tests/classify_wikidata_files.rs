use file_type::FileType;

#[cfg(feature = "wikidata")]
fn data_dir() -> std::path::PathBuf {
    let crate_dir: &str = env!("CARGO_MANIFEST_DIR");
    std::path::PathBuf::from(crate_dir)
        .join("..")
        .join("testdata")
        .join("wikidata")
}

#[cfg(feature = "wikidata")]
fn test_file(file_name: &str) -> anyhow::Result<(String, &FileType)> {
    let data_dir = data_dir();
    let path = data_dir.join(file_name);
    let file_name = path
        .file_name()
        .expect("file name")
        .to_string_lossy()
        .to_string();
    let file_name = file_name.split('.').next().expect("split").to_string();
    let parts: Vec<&str> = file_name.split('-').collect();
    let id = format!("{}/{}", parts[0], parts[1]);
    let file_type = FileType::try_from_file_sync(path)?;
    Ok((id, file_type))
}

#[cfg(feature = "wikidata")]
#[test]
fn test_file_classification() -> anyhow::Result<()> {
    let data_dir = data_dir();
    let mut passed_tests = 0;
    let mut errored_tests = 0;

    for entry in std::fs::read_dir(data_dir)? {
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
    assert!(passed_tests > 11000);
    Ok(())
}

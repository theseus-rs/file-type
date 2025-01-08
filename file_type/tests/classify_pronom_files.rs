use anyhow::Result;
use file_type::FileType;
use std::path::PathBuf;
use walkdir::WalkDir;

const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const IGNORED: [&str; 58] = [
    "fmt/62",
    "fmt/63",
    "fmt/64",
    "fmt/65",
    "fmt/66",
    "fmt/67",
    "fmt/68",
    "fmt/69",
    "fmt/70",
    "fmt/71",
    "fmt/72",
    "fmt/73",
    "fmt/74",
    "fmt/75",
    "fmt/76",
    "fmt/77",
    "fmt/78",
    "fmt/79",
    "fmt/160",
    "fmt/276",
    "fmt/301",
    "fmt/302",
    "fmt/433",
    "fmt/435",
    "fmt/507",
    "fmt/532",
    "fmt/558",
    "fmt/570",
    "fmt/577",
    "fmt/578",
    "fmt/579",
    "fmt/580",
    "fmt/581",
    "fmt/582",
    "fmt/890",
    "fmt/891",
    "fmt/950",
    "fmt/1014",
    "fmt/1062",
    "fmt/1039",
    "fmt/1105",
    "fmt/1163",
    "fmt/1199",
    "fmt/1389",
    "fmt/1451",
    "fmt/1463",
    "fmt/1464",
    "fmt/1725",
    "fmt/1739",
    "fmt/1757",
    "fmt/1796",
    "fmt/1845",
    "fmt/1855",
    "fmt/1871",
    "fmt/2008",
    "fmt/2009",
    "x-fmt/280",
    "x-fmt/365",
];

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
    let mut ignored_tests = 0;

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

        if IGNORED.contains(&id.as_str()) {
            if file_type.id() == id {
                eprintln!(
                    "IGNORED(PASSING) file_type.id()={}, id={id}: {file_name}",
                    file_type.id(),
                );
            } else {
                eprintln!(
                    "IGNORED(ERROR) file_type.id()={}, id={id}: {file_name}",
                    file_type.id(),
                );
            }
            ignored_tests += 1;
            continue;
        }
        assert_eq!(file_type.id(), id);
        passed_tests += 1;
    }

    println!("Passed: {passed_tests}");
    println!("Ignored: {ignored_tests}");
    Ok(())
}

#[tokio::test]
async fn test_single_file_classification() -> Result<()> {
    let (id, file_type) = test_file("fmt-708-signature-id-831.wav").await?;
    assert_eq!(file_type.id(), id);
    Ok(())
}

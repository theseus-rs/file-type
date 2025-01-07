use anyhow::Result;
use file_type::FileType;
use std::path::PathBuf;

const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn data_dir() -> PathBuf {
    PathBuf::from(CRATE_DIR)
        .join("..")
        .join("testdata")
        .join("custom")
}

async fn test_file(
    file_name: &str,
    expected_id: &str,
    expected_media_type: Option<&str>,
) -> Result<()> {
    let data_dir = data_dir();
    let path = data_dir.join(file_name);
    let file_type = FileType::try_from_file(path).await?;
    assert_eq!(file_type.id(), expected_id);
    match expected_media_type {
        Some(expected_media_type) => {
            assert!(file_type.media_types().contains(&expected_media_type));
        }
        None => {
            assert!(file_type.media_types().is_empty());
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_arrow_file() -> Result<()> {
    test_file(
        "users.arrow",
        "custom/1",
        Some("application/vnd.apache.arrow.file"),
    )
    .await
}

#[tokio::test]
async fn test_avro_file() -> Result<()> {
    test_file(
        "users.avro",
        "custom/2",
        Some("application/vnd.apache.avro.file"),
    )
    .await
}

#[tokio::test]
async fn test_csv_file() -> Result<()> {
    test_file("users.csv", "x-fmt/18", Some("text/csv")).await
}

#[tokio::test]
async fn test_duckdb_file() -> Result<()> {
    test_file(
        "users.duckdb",
        "custom/3",
        Some("application/vnd.duckdb.file"),
    )
    .await
}

#[tokio::test]
async fn test_json_file() -> Result<()> {
    test_file("users.json", "fmt/817", Some("application/json")).await
}

#[tokio::test]
async fn test_jsonl_file() -> Result<()> {
    test_file("users.jsonl", "custom/4", Some("application/jsonl")).await
}

#[tokio::test]
async fn test_parquet_file() -> Result<()> {
    test_file(
        "users.parquet",
        "custom/5",
        Some("application/vnd.apache.parquet"),
    )
    .await
}

#[tokio::test]
async fn test_sqlite3_file() -> Result<()> {
    test_file("users.sqlite3", "fmt/729", Some("application/x-sqlite3")).await
}

#[tokio::test]
async fn test_tsv_file() -> Result<()> {
    test_file("users.tsv", "x-fmt/13", Some("text/tab-separated-values")).await
}

#[tokio::test]
async fn test_xml_file() -> Result<()> {
    test_file("users.xml", "fmt/101", Some("text/xml")).await
}

#[tokio::test]
async fn test_yaml_file() -> Result<()> {
    test_file("users.yaml", "fmt/818", None).await
}

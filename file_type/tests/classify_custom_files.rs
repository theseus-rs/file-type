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

fn test_file(file_name: &str, expected_id: &str, expected_media_type: Option<&str>) -> Result<()> {
    let data_dir = data_dir();
    let path = data_dir.join(file_name);
    let file_type = FileType::try_from_file_sync(path)?;
    assert_eq!(file_type.id(), expected_id);
    let media_types = file_type.media_types();
    match expected_media_type {
        Some(expected_media_type) => {
            assert!(media_types.contains(&expected_media_type));
        }
        None => {
            assert!(media_types.is_empty());
        }
    }
    Ok(())
}

#[test]
fn test_arrow_file() -> Result<()> {
    test_file(
        "users.arrow",
        "custom/1",
        Some("application/vnd.apache.arrow.file"),
    )
}

#[test]
fn test_avro_file() -> Result<()> {
    test_file(
        "users.avro",
        "custom/2",
        Some("application/vnd.apache.avro.file"),
    )
}

#[test]
fn test_csv_file() -> Result<()> {
    test_file("users.csv", "pronom/45", Some("text/csv"))
}

#[test]
fn test_duckdb_file() -> Result<()> {
    test_file(
        "users.duckdb",
        "custom/3",
        Some("application/vnd.duckdb.file"),
    )
}

#[test]
fn test_json_file() -> Result<()> {
    test_file("users.json", "pronom/1617", Some("application/json"))
}

#[test]
fn test_jsonl_file() -> Result<()> {
    test_file("users.jsonl", "custom/4", Some("application/jsonl"))
}

#[test]
fn test_ods_file() -> Result<()> {
    test_file(
        "users.ods",
        "pronom/780",
        Some("application/vnd.oasis.opendocument.spreadsheet"),
    )
}

#[test]
fn test_parquet_file() -> Result<()> {
    test_file(
        "users.parquet",
        "custom/5",
        Some("application/vnd.apache.parquet"),
    )
}

#[test]
fn test_sqlite3_file() -> Result<()> {
    test_file(
        "users.sqlite3",
        "pronom/1528",
        Some("application/x-sqlite3"),
    )
}

#[test]
fn test_tsv_file() -> Result<()> {
    test_file("users.tsv", "pronom/40", Some("text/tab-separated-values"))
}

#[test]
fn test_xlsx_file() -> Result<()> {
    test_file(
        "users.xlsx",
        "pronom/940",
        Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
    )
}

#[test]
fn test_xml_file() -> Result<()> {
    test_file("users.xml", "pronom/638", Some("text/xml"))
}

#[test]
fn test_yaml_file() -> Result<()> {
    test_file("users.yaml", "pronom/1618", None)
}

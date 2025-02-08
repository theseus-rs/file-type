use file_type::format::SourceType;
use file_type::FileType;

#[cfg(feature = "custom")]
fn data_dir() -> std::path::PathBuf {
    let crate_dir: &str = env!("CARGO_MANIFEST_DIR");
    std::path::PathBuf::from(crate_dir)
        .join("..")
        .join("testdata")
        .join("custom")
}

#[cfg(feature = "custom")]
fn test_file(
    file_name: &str,
    expected_id: usize,
    expected_source_type: &SourceType,
    expected_media_type: Option<&str>,
) -> anyhow::Result<()> {
    let data_dir = data_dir();
    let path = data_dir.join(file_name);
    let file_type = FileType::try_from_file_sync(path)?;
    assert_eq!(file_type.id(), expected_id);
    assert_eq!(file_type.source_type(), expected_source_type);
    let media_types = file_type.media_types();
    match expected_media_type {
        Some(expected_media_type) => {
            assert!(media_types.contains(&expected_media_type));
        }
        None => {
            let empty: Vec<&str> = Vec::new();
            assert_eq!(media_types, empty);
        }
    }
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_arrow_file() -> anyhow::Result<()> {
    test_file(
        "users.arrow",
        1,
        &SourceType::Custom,
        Some("application/vnd.apache.arrow.file"),
    )
}

#[cfg(feature = "custom")]
#[test]
fn test_avro_file() -> anyhow::Result<()> {
    test_file(
        "users.avro",
        2,
        &SourceType::Custom,
        Some("application/vnd.apache.avro.file"),
    )
}

#[cfg(feature = "custom")]
#[test]
fn test_csv_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file("users.csv", 45, &SourceType::Pronom, Some("text/csv"))?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file("users.csv", 935809, &SourceType::Wikidata, Some("text/csv"))?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_duckdb_file() -> anyhow::Result<()> {
    test_file(
        "users.duckdb",
        3,
        &SourceType::Custom,
        Some("application/vnd.duckdb.file"),
    )
}

#[cfg(feature = "custom")]
#[test]
fn test_json_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file(
        "users.json",
        1_617,
        &SourceType::Pronom,
        Some("application/json"),
    )?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file(
        "users.json",
        2_063,
        &SourceType::Wikidata,
        Some("application/json"),
    )?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_jsonl_file() -> anyhow::Result<()> {
    test_file(
        "users.jsonl",
        4,
        &SourceType::Custom,
        Some("application/jsonl"),
    )
}

#[cfg(feature = "custom")]
#[test]
fn test_ods_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file(
        "users.ods",
        780,
        &SourceType::Pronom,
        Some("application/vnd.oasis.opendocument.spreadsheet"),
    )?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file(
        "users.ods",
        27_203_692,
        &SourceType::Wikidata,
        Some("application/vnd.oasis.opendocument.spreadsheet"),
    )?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_parquet_file() -> anyhow::Result<()> {
    test_file(
        "users.parquet",
        5,
        &SourceType::Custom,
        Some("application/vnd.apache.parquet"),
    )
}

#[cfg(feature = "custom")]
#[test]
fn test_sqlite3_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file(
        "users.sqlite3",
        1_528,
        &SourceType::Pronom,
        Some("application/x-sqlite3"),
    )?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file(
        "users.sqlite3",
        28_600_453,
        &SourceType::Wikidata,
        Some("application/vnd.sqlite3"),
    )?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_tsv_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file(
        "users.tsv",
        40,
        &SourceType::Pronom,
        Some("text/tab-separated-values"),
    )?;
    // Wikidata is currently mis-classifying this file as wikidata/1194435 instead of wikidata/3513566
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_xlsx_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file(
        "users.xlsx",
        940,
        &SourceType::Pronom,
        Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
    )?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file(
        "users.xlsx",
        3_570_403,
        &SourceType::Wikidata,
        Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
    )?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_xml_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file("users.xml", 638, &SourceType::Pronom, Some("text/xml"))?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file(
        "users.xml",
        59_851_322,
        &SourceType::Wikidata,
        Some("text/xml"),
    )?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_yaml_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file("users.yaml", 1_618, &SourceType::Pronom, None)?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file(
        "users.yaml",
        281_876,
        &SourceType::Wikidata,
        Some("application/yaml"),
    )?;
    Ok(())
}

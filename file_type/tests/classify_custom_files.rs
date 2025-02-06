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
    expected_id: &str,
    expected_media_type: Option<&str>,
) -> anyhow::Result<()> {
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
        "custom/1",
        Some("application/vnd.apache.arrow.file"),
    )
}

#[cfg(feature = "custom")]
#[test]
fn test_avro_file() -> anyhow::Result<()> {
    test_file(
        "users.avro",
        "custom/2",
        Some("application/vnd.apache.avro.file"),
    )
}

#[cfg(feature = "custom")]
#[test]
fn test_csv_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file("users.csv", "pronom/45", Some("text/csv"))?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file("users.csv", "wikidata/935809", Some("text/csv"))?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_duckdb_file() -> anyhow::Result<()> {
    test_file(
        "users.duckdb",
        "custom/3",
        Some("application/vnd.duckdb.file"),
    )
}

#[cfg(feature = "custom")]
#[test]
fn test_json_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file("users.json", "pronom/1617", Some("application/json"))?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file("users.json", "wikidata/2063", Some("application/json"))?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_jsonl_file() -> anyhow::Result<()> {
    test_file("users.jsonl", "custom/4", Some("application/jsonl"))
}

#[cfg(feature = "custom")]
#[test]
fn test_ods_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file(
        "users.ods",
        "pronom/780",
        Some("application/vnd.oasis.opendocument.spreadsheet"),
    )?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file(
        "users.ods",
        "wikidata/27203692",
        Some("application/vnd.oasis.opendocument.spreadsheet"),
    )?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_parquet_file() -> anyhow::Result<()> {
    test_file(
        "users.parquet",
        "custom/5",
        Some("application/vnd.apache.parquet"),
    )
}

#[cfg(feature = "custom")]
#[test]
fn test_sqlite3_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file(
        "users.sqlite3",
        "pronom/1528",
        Some("application/x-sqlite3"),
    )?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file(
        "users.sqlite3",
        "wikidata/28600453",
        Some("application/vnd.sqlite3"),
    )?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_tsv_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file("users.tsv", "pronom/40", Some("text/tab-separated-values"))?;
    // Wikidata is currently mis-classifying this file as wikidata/1194435 instead of wikidata/3513566
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_xlsx_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file(
        "users.xlsx",
        "pronom/940",
        Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
    )?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file(
        "users.xlsx",
        "wikidata/3570403",
        Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
    )?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_xml_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file("users.xml", "pronom/638", Some("text/xml"))?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file("users.xml", "wikidata/59851322", Some("text/xml"))?;
    Ok(())
}

#[cfg(feature = "custom")]
#[test]
fn test_yaml_file() -> anyhow::Result<()> {
    #[cfg(feature = "pronom")]
    test_file("users.yaml", "pronom/1618", None)?;
    #[cfg(all(not(feature = "pronom"), feature = "wikidata"))]
    test_file("users.yaml", "wikidata/281876", Some("application/yaml"))?;
    Ok(())
}

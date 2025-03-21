use file_type::FileType;
use file_type::format::SourceType;

#[cfg(feature = "wikidata")]
fn data_dir() -> std::path::PathBuf {
    let crate_dir: &str = env!("CARGO_MANIFEST_DIR");
    std::path::PathBuf::from(crate_dir)
        .join("..")
        .join("test_data")
        .join("custom")
}

#[cfg(feature = "wikidata")]
fn test_file(
    file_name: &str,
    expected_id: usize,
    expected_source_type: &SourceType,
    expected_media_type: Option<&str>,
) -> anyhow::Result<()> {
    let data_dir = data_dir();
    let path = data_dir.join(file_name);
    let file_type = FileType::try_from_file(path)?;
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

#[cfg(feature = "wikidata")]
#[test]
fn test_arrow_file() -> anyhow::Result<()> {
    test_file(
        "users.arrow",
        133_285_299,
        &SourceType::Wikidata,
        Some("application/vnd.apache.arrow.file"),
    )
}

#[cfg(feature = "wikidata")]
#[test]
fn test_avro_file() -> anyhow::Result<()> {
    test_file("users.avro", 105_855_052, &SourceType::Wikidata, None)
}

#[cfg(feature = "wikidata")]
#[test]
fn test_csv_file() -> anyhow::Result<()> {
    test_file(
        "users.csv",
        935_809,
        &SourceType::Wikidata,
        Some("text/csv"),
    )?;
    Ok(())
}

#[cfg(feature = "wikidata")]
#[test]
fn test_duckdb_file() -> anyhow::Result<()> {
    test_file("users.duckdb", 133_271_766, &SourceType::Wikidata, None)
}

#[cfg(feature = "wikidata")]
#[test]
fn test_json_file() -> anyhow::Result<()> {
    test_file(
        "users.json",
        2_063,
        &SourceType::Wikidata,
        Some("application/json"),
    )?;
    Ok(())
}

#[cfg(feature = "wikidata")]
#[test]
fn test_jsonl_file() -> anyhow::Result<()> {
    test_file(
        "users.jsonl",
        111_841_144,
        &SourceType::Wikidata,
        Some("application/jsonl"),
    )
}

#[cfg(feature = "wikidata")]
#[test]
fn test_ods_file() -> anyhow::Result<()> {
    test_file(
        "users.ods",
        27_203_692,
        &SourceType::Wikidata,
        Some("application/vnd.oasis.opendocument.spreadsheet"),
    )?;
    Ok(())
}

#[cfg(feature = "wikidata")]
#[test]
fn test_orc_file() -> anyhow::Result<()> {
    test_file("users.orc", 60_767_426, &SourceType::Wikidata, None)?;
    Ok(())
}

#[cfg(feature = "wikidata")]
#[test]
fn test_parquet_file() -> anyhow::Result<()> {
    test_file(
        "users.parquet",
        28_915_683,
        &SourceType::Wikidata,
        Some("application/vnd.apache.parquet"),
    )
}

#[cfg(feature = "wikidata")]
#[test]
fn test_sqlite3_file() -> anyhow::Result<()> {
    test_file(
        "users.sqlite3",
        28_600_453,
        &SourceType::Wikidata,
        Some("application/vnd.sqlite3"),
    )?;
    Ok(())
}

#[cfg(feature = "wikidata")]
#[test]
fn test_tsv_file() -> anyhow::Result<()> {
    // Wikidata is currently mis-classifying this file as wikidata/1194435 instead of wikidata/3513566
    test_file(
        "users.tsv",
        1_194_435,
        &SourceType::Wikidata,
        Some("video/mp2t"),
    )?;
    Ok(())
}

#[cfg(feature = "wikidata")]
#[test]
fn test_xlsx_file() -> anyhow::Result<()> {
    test_file(
        "users.xlsx",
        3_570_403,
        &SourceType::Wikidata,
        Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
    )?;
    Ok(())
}

#[cfg(feature = "wikidata")]
#[test]
fn test_xml_file() -> anyhow::Result<()> {
    test_file(
        "users.xml",
        59_851_322,
        &SourceType::Wikidata,
        Some("text/xml"),
    )?;
    Ok(())
}

#[cfg(feature = "wikidata")]
#[test]
fn test_yaml_file() -> anyhow::Result<()> {
    test_file(
        "users.yaml",
        281_876,
        &SourceType::Wikidata,
        Some("application/yaml"),
    )?;
    Ok(())
}

//
// Compression formats
//

#[cfg(feature = "wikidata")]
#[test]
fn test_compression_br() -> anyhow::Result<()> {
    test_file(
        "users.csv.br",
        133_452_018,
        &SourceType::Wikidata,
        Some("application/x-br"),
    )
}

#[cfg(feature = "wikidata")]
#[test]
fn test_compression_bz2() -> anyhow::Result<()> {
    // Wikidata is currently mis-classifying this file as wikidata/25822040 instead of wikidata/27866052
    test_file(
        "users.csv.bz2",
        25_822_040,
        &SourceType::Wikidata,
        Some("application/vnd.openstreetmap.data+xml"),
    )
}

#[cfg(feature = "wikidata")]
#[test]
fn test_compression_gz() -> anyhow::Result<()> {
    test_file(
        "users.csv.gz",
        10_287_816,
        &SourceType::Wikidata,
        Some("application/gzip"),
    )
}

#[cfg(feature = "wikidata")]
#[test]
fn test_compression_lz() -> anyhow::Result<()> {
    test_file(
        "users.csv.lz",
        105_854_729,
        &SourceType::Wikidata,
        Some("application/x-lzip"),
    )
}

#[cfg(feature = "wikidata")]
#[test]
fn test_compression_lz4() -> anyhow::Result<()> {
    test_file(
        "users.csv.lz4",
        28_770_292,
        &SourceType::Wikidata,
        Some("application/x-lz4"),
    )
}

#[cfg(feature = "wikidata")]
#[test]
fn test_compression_xz() -> anyhow::Result<()> {
    test_file(
        "users.csv.xz",
        162_839,
        &SourceType::Wikidata,
        Some("application/x-xz"),
    )
}

#[cfg(feature = "wikidata")]
#[test]
fn test_compression_z() -> anyhow::Result<()> {
    test_file(
        "users.csv.Z",
        29_209_269,
        &SourceType::Wikidata,
        Some("application/x-compress"),
    )
}

#[cfg(feature = "wikidata")]
#[test]
fn test_compression_zst() -> anyhow::Result<()> {
    test_file(
        "users.csv.zst",
        105_853_477,
        &SourceType::Wikidata,
        Some("application/octet-stream"),
    )
}

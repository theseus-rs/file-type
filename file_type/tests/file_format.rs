use anyhow::Result;
use file_type::format::FileFormat;
use std::path::PathBuf;
use tokio::time::Instant;

const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

async fn file_format(format_type: &str, file_name: &str) -> Result<FileFormat> {
    let path = PathBuf::from(CRATE_DIR)
        .join("data")
        .join(format_type)
        .join(file_name);
    let xml = tokio::fs::read_to_string(path).await?;
    let file_format = quick_xml::de::from_str(xml.as_str())?;
    Ok(file_format)
}

#[tokio::test]
async fn test_variable_format() -> Result<()> {
    let file_format = file_format("pronom", "fmt-63.xml").await?;
    let length = 1 << 31;
    let bytes = vec![0; length];
    let start = Instant::now();
    let _ = file_format.is_match(&bytes);
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 10);
    Ok(())
}

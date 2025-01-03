#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
use file_type::pronom::{DocumentIdentifier, ExternalSignature, FileFormat};
use file_type::FileType;
use jiff::civil::Date;
use jiff::tz::TimeZone;
use jiff::Timestamp;
use quick_xml::se::Serializer;
use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{info, warn};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, EnvFilter};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const MIME_TYPES_URL: &str =
    "https://raw.githubusercontent.com/apache/httpd/refs/heads/trunk/docs/conf/mime.types";

/// Downloads Apache HTTPD mime type data and saves it to the file_type crate data directory.
#[tokio::main]
async fn main() -> Result<()> {
    initialize_tracing();
    export_data().await?;
    Ok(())
}

fn initialize_tracing() {
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_names(true)
        .with_timer(fmt::time::uptime())
        .compact();
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("APACHE_HTTPD_LOG")
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .fmt_fields(fmt::format::DefaultFields::new())
        .event_format(format)
        .init();
}

async fn export_data() -> Result<()> {
    let new_dir = PathBuf::from(CRATE_DIR)
        .join("..")
        .join("..")
        .join("file_type")
        .join("data")
        .join("apache-httpd");
    fs::create_dir_all(&new_dir).await?;

    let client = Client::new();
    let response = client
        .get(MIME_TYPES_URL)
        .header("User-Agent", format!("{CRATE_NAME}/{CRATE_VERSION}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await?;

    let mime_types_data = response.text().await?;
    let mime_types = parse_mime_types(mime_types_data);
    let file_formats = process_mime_types(mime_types)?;
    for file_format in file_formats {
        store_file_format(&file_format, &new_dir).await?;
    }

    Ok(())
}

fn parse_mime_types<S: AsRef<str>>(data: S) -> HashMap<String, Vec<String>> {
    let mut mime_types: HashMap<String, Vec<String>> = HashMap::new();

    for line in data.as_ref().lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            warn!("Invalid line: {line}");
            continue;
        }

        let mime_type = parts[0];
        let extensions = &parts[1..];

        let extensions = extensions
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();

        mime_types.insert(mime_type.to_string(), extensions);
    }

    mime_types
}

fn process_mime_types(mime_types: HashMap<String, Vec<String>>) -> Result<Vec<FileFormat>> {
    let mut file_formats = Vec::new();

    for (mime_type, extensions) in mime_types {
        let exists = extensions
            .iter()
            .any(|extension| mime_type_exists(&mime_type, extension));
        if exists {
            continue;
        }

        let external_signatures = extensions
            .iter()
            .enumerate()
            .map(|(index, extension)| {
                ExternalSignature::new(index, extension.as_str(), "File extension")
            })
            .collect::<Vec<ExternalSignature>>();

        let mut hasher = DefaultHasher::new();
        mime_type.hash(&mut hasher);
        let hash = usize::try_from(hasher.finish())?;
        let puid = format!("apache-httpd/{hash}");
        let name = mime_type.as_str().split_once('/').unwrap_or_default().1;
        let name = name
            .trim_start_matches("vnd.")
            .trim_start_matches("x-")
            .replace(['-', '+', '.'], " ");
        let file_format = FileFormat::new(
            hash,
            name.as_str(),
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            None,
            None,
            0,
            "",
            Date::from(Timestamp::now().to_zoned(TimeZone::UTC)),
            "",
            Date::from(Timestamp::now().to_zoned(TimeZone::UTC)),
            "",
            "",
            "",
            vec![
                DocumentIdentifier::new(puid.as_str(), "PUID"),
                DocumentIdentifier::new(mime_type.as_str(), "MIME"),
            ],
            vec![],
            external_signatures,
            vec![],
            vec![],
            vec![],
        );
        file_formats.push(file_format);
    }

    Ok(file_formats)
}

/// Check if a mime type and extension already exist in the file type data.
fn mime_type_exists<S: AsRef<str>>(mime_type: S, extension: S) -> bool {
    let mime_type = mime_type.as_ref();
    let extension = extension.as_ref();
    for file_type in FileType::from_media_type(mime_type) {
        if file_type.extensions().contains(&extension) {
            return true;
        }
    }

    false
}

async fn store_file_format(file_format: &FileFormat, output_dir: &Path) -> Result<()> {
    let file_name = format!("{}.xml", file_format.puid().replace('/', "-"));
    info!("{file_name}: {}", file_format.name());
    let file_name = output_dir.join(file_name);
    let mut buffer = String::new();
    let mut serializer = Serializer::new(&mut buffer);
    serializer.indent(' ', 2);
    file_format.serialize(serializer)?;
    let mut file = File::create(file_name).await?;
    file.write_all(buffer.as_bytes()).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let result = main();
        assert!(result.is_ok());
    }
}

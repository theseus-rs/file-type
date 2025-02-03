#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
use csv::ReaderBuilder;
use file_type::format::{FileFormat, Source, SourceType};
use reqwest::Client;
use std::collections::HashMap;
use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{info, warn};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, EnvFilter};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const MEDIA_TYPES_URL: &str = "https://www.iana.org/assignments/media-types";
const CATEGORIES: &[&str] = &[
    "application",
    "audio",
    "font",
    "haptics",
    "image",
    "message",
    "model",
    "multipart",
    "text",
    "video",
];

/// Downloads IANA mime type data and saves it to the file_type crate data directory.
#[tokio::main]
async fn main() -> Result<()> {
    initialize_tracing();
    let dry_run = env::var("DRY_RUN").is_ok();
    execute(dry_run).await?;
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
        .with_env_var("IANA_LOG")
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .fmt_fields(fmt::format::DefaultFields::new())
        .event_format(format)
        .init();
}

async fn execute(dry_run: bool) -> Result<()> {
    let source_dir = PathBuf::from(CRATE_DIR)
        .join("..")
        .join("..")
        .join("file_type")
        .join("src")
        .join("sources")
        .join("iana");

    let client = Client::new();
    let mut file_formats = HashMap::new();
    for category in CATEGORIES {
        let url = format!("{MEDIA_TYPES_URL}/{category}.csv");
        info!("Downloading {category}: {url}");
        let response = client
            .get(url)
            .header("User-Agent", format!("{CRATE_NAME}/{CRATE_VERSION}"))
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        let data = response.text().await?;
        parse_media_types(data, &mut file_formats)?;
    }

    let mut file_formats: Vec<FileFormat> = file_formats.values().cloned().collect();
    file_formats.sort_by_key(|file_format| file_format.id);
    generate_source_code(&file_formats, &source_dir, dry_run).await?;
    Ok(())
}

fn parse_media_types<S: AsRef<str>>(
    data: S,
    file_formats: &mut HashMap<u32, FileFormat>,
) -> Result<()> {
    let data = data.as_ref().as_bytes();
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(data);
    for result in reader.records() {
        let record = result?;
        if record.len() < 2 {
            continue;
        }

        let name = record.get(0).expect("missing name");
        let media_type = record.get(1).expect("missing media type").to_string();

        let mut hasher = DefaultHasher::new();
        media_type.hash(&mut hasher);
        let hash = hasher.finish();
        let high = (hash >> 32) as u32;
        #[expect(clippy::cast_possible_truncation)]
        let low = hash as u32;
        let id = high ^ low;

        let media_type: &'static str = Box::leak(media_type.into_boxed_str());
        let media_types = vec![media_type];

        let file_format = FileFormat {
            id: id as usize,
            name: Box::leak(name.to_string().into_boxed_str()),
            source_type: SourceType::Iana,
            media_types: Box::leak(media_types.into_boxed_slice()),
            ..Default::default()
        };
        if let Some(file_format) = file_formats.insert(id, file_format) {
            warn!("Duplicate ID: {id}, FileFormat: {file_format:?}");
        }
    }

    Ok(())
}

async fn generate_source_code(
    file_formats: &Vec<FileFormat>,
    source_dir: &Path,
    dry_run: bool,
) -> Result<()> {
    // Generate the module file
    let mut source_code = vec!["use crate::format::FileFormat;".to_string(), String::new()];
    for file_format in file_formats {
        let name = format!("iana_{}", file_format.id);
        source_code.push(format!("mod {name};"));
    }
    source_code.push(String::new());
    source_code.push("pub(crate) const FILE_FORMATS: &[&FileFormat] = &[".to_string());
    for file_format in file_formats {
        let name = format!("iana_{}", file_format.id);
        source_code.push(format!("    &{}::{},", name, name.to_uppercase()));
    }
    source_code.push("];".to_string());
    source_code.push(String::new());
    let file_name = source_dir.join("mod.rs");
    if dry_run {
        warn!("[dry-run] Writing {}", file_name.to_string_lossy());
    } else {
        info!("Writing {}", file_name.to_string_lossy());
        let mut source_file = File::create(file_name).await?;
        source_file
            .write_all(source_code.join("\n").as_bytes())
            .await?;
    }

    // Generate source files for each file format
    for file_format in file_formats {
        let name = format!("iana_{}", file_format.id);
        let source_code = [
            "use crate::format::{FileFormat, SourceType};".to_string(),
            String::new(),
            format!(
                "pub(crate) const {}: FileFormat = {};",
                name.to_uppercase(),
                file_format.to_source(),
            ),
        ]
        .join("\n");
        let file_name = source_dir.join(format!("{name}.rs"));
        if dry_run {
            warn!("[dry-run] Writing {}", file_name.to_string_lossy());
        } else {
            info!("Writing {}", file_name.to_string_lossy());
            let mut source_file = File::create(file_name).await?;
            source_file.write_all(source_code.as_bytes()).await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        env::set_var("DRY_RUN", "true");
        let result = main();
        assert!(result.is_ok());
    }
}

#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
use file_type::format::{FileFormat, SourceType};
use reqwest::blocking::Client;
use source_generator::generate;
use std::collections::HashMap;
use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;
use std::time::Duration;
use tracing::warn;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, EnvFilter};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const MIME_TYPES_URL: &str =
    "https://raw.githubusercontent.com/apache/httpd/refs/heads/trunk/docs/conf/mime.types";

/// Downloads Apache HTTPD mime type data and saves it to the `file_type` crate data directory.
fn main() -> Result<()> {
    initialize_tracing();
    let dry_run = env::var("DRY_RUN").is_ok();
    execute(dry_run)?;
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
        .with_env_var("HTTPD_LOG")
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .fmt_fields(fmt::format::DefaultFields::new())
        .event_format(format)
        .init();
}

fn execute(dry_run: bool) -> Result<()> {
    let source_dir = PathBuf::from(CRATE_DIR)
        .join("..")
        .join("..")
        .join("file_type")
        .join("src")
        .join("sources")
        .join("httpd");

    let client = Client::new();
    let response = client
        .get(MIME_TYPES_URL)
        .header("User-Agent", format!("{CRATE_NAME}/{CRATE_VERSION}"))
        .timeout(Duration::from_secs(30))
        .send()?;

    let mime_types_data = response.text()?;
    let mime_types = parse_mime_types(mime_types_data);
    let mut file_formats = process_mime_types(mime_types);
    file_formats.sort_by_key(|file_format| file_format.id);
    generate(&file_formats, &source_dir, dry_run)?;
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

fn process_mime_types(mime_types: HashMap<String, Vec<String>>) -> Vec<FileFormat> {
    let mut file_formats = HashMap::new();

    for (mime_type, extensions) in mime_types {
        let extensions = extensions
            .into_iter()
            .map(|extension| {
                let extension: &'static str = Box::leak(extension.into_boxed_str());
                extension
            })
            .collect::<Vec<&str>>();
        let media_types = vec![mime_type.to_string()]
            .into_iter()
            .map(|media_type| {
                let media_type: &'static str = Box::leak(media_type.into_boxed_str());
                media_type
            })
            .collect::<Vec<&str>>();
        let mut hasher = DefaultHasher::new();
        mime_type.hash(&mut hasher);
        let hash = hasher.finish();
        let high = (hash >> 32) as u32;
        #[expect(clippy::cast_possible_truncation)]
        let low = hash as u32;
        let id = high ^ low;
        let name = mime_type.as_str().split_once('/').unwrap_or_default().1;
        let name = name
            .trim_start_matches("vnd.")
            .trim_start_matches("x-")
            .replace(['-', '+', '.'], " ");
        let file_format = FileFormat {
            id: id as usize,
            source_type: SourceType::Httpd,
            name: Box::leak(name.into_boxed_str()),
            extensions: Box::leak(extensions.into_boxed_slice()),
            media_types: Box::leak(media_types.into_boxed_slice()),
            ..Default::default()
        };
        if let Some(file_format) = file_formats.insert(id, file_format) {
            warn!("Duplicate ID: {id}, FileFormat: {file_format:?}");
        }
    }

    file_formats.values().cloned().collect()
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

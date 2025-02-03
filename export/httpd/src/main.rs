#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
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
const MIME_TYPES_URL: &str =
    "https://raw.githubusercontent.com/apache/httpd/refs/heads/trunk/docs/conf/mime.types";

/// Downloads Apache HTTPD mime type data and saves it to the file_type crate data directory.
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
        .with_env_var("HTTPD_LOG")
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
        .join("httpd");

    let client = Client::new();
    let response = client
        .get(MIME_TYPES_URL)
        .header("User-Agent", format!("{CRATE_NAME}/{CRATE_VERSION}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await?;

    let mime_types_data = response.text().await?;
    let mime_types = parse_mime_types(mime_types_data);
    let mut file_formats = process_mime_types(mime_types);
    file_formats.sort_by_key(|file_format| file_format.id);
    generate_source_code(&file_formats, &source_dir, dry_run).await?;
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

async fn generate_source_code(
    file_formats: &Vec<FileFormat>,
    source_dir: &Path,
    dry_run: bool,
) -> Result<()> {
    // Generate the module file
    let mut source_code = vec!["use crate::format::FileFormat;".to_string(), String::new()];
    for file_format in file_formats {
        let name = format!("httpd_{}", file_format.id);
        source_code.push(format!("mod {name};"));
    }
    source_code.push(String::new());
    source_code.push("pub(crate) const FILE_FORMATS: &[&FileFormat] = &[".to_string());
    for file_format in file_formats {
        let name = format!("httpd_{}", file_format.id);
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
        let name = format!("httpd_{}", file_format.id);
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

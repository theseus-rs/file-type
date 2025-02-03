#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
use file_type::format::{FileFormat, Source, SourceType};
use reqwest::Client;
use serde_json::Value;
use std::env;
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
    "https://raw.githubusercontent.com/github-linguist/linguist/refs/heads/main/lib/linguist/languages.yml";

struct Language {
    id: usize,
    name: String,
    mime_type: String,
    extensions: Vec<String>,
}

/// Downloads Linguist mime type data and saves it to the file_type crate data directory.
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
        .with_env_var("LINGUIST_LOG")
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
        .join("linguist");

    let client = Client::new();
    let response = client
        .get(MIME_TYPES_URL)
        .header("User-Agent", format!("{CRATE_NAME}/{CRATE_VERSION}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await?;

    let language_data = response.text().await?;
    let mime_types = parse_languages(language_data)?;
    let mut file_formats = process_languages(mime_types);
    file_formats.sort_by_key(|file_format| file_format.id);
    generate_source_code(&file_formats, &source_dir, dry_run).await?;
    Ok(())
}

fn convert_yaml_to_json<S: AsRef<str>>(yaml: S) -> Result<Value> {
    let json: Value = serde_yaml::from_str(yaml.as_ref())?;
    Ok(json)
}

fn parse_languages<S: AsRef<str>>(yaml: S) -> Result<Vec<Language>> {
    let mut languages = Vec::new();
    let json = convert_yaml_to_json(yaml)?;
    let language_map = json.as_object().expect("Invalid JSON");

    for (name, language) in language_map {
        let language = language.as_object().expect("Invalid language map");
        let id = language.get("language_id").expect("Invalid language id");
        let id = id.as_u64().expect("Invalid language id");
        let mime_type = match language.get("codemirror_mime_type") {
            Some(mime_type) => mime_type.as_str().expect("Invalid mime type").to_string(),
            None => String::new(),
        };
        let mut extensions = match language.get("extensions") {
            Some(Value::Array(extensions)) => extensions
                .iter()
                .map(|extension| {
                    let extension = extension.as_str().expect("Invalid extension").to_string();
                    extension.trim_start_matches('.').to_string()
                })
                .collect(),
            _ => Vec::new(),
        };
        extensions.sort();

        let language = Language {
            id: usize::try_from(id)?,
            name: name.to_string(),
            mime_type,
            extensions,
        };
        languages.push(language);
    }

    Ok(languages)
}

fn process_languages(languages: Vec<Language>) -> Vec<FileFormat> {
    let mut file_formats = Vec::new();

    for language in languages {
        let extensions = &language.extensions;
        let extensions = extensions
            .iter()
            .map(|extension| {
                let extension: &'static str = Box::leak(extension.clone().into_boxed_str());
                extension
            })
            .collect::<Vec<&str>>();
        let mime_type = &language.mime_type;
        let mut media_types = Vec::new();
        if !mime_type.is_empty() {
            media_types.push(mime_type.to_string());
        }
        let media_types = media_types
            .iter()
            .map(|media_type| {
                let media_type: &'static str = Box::leak(media_type.clone().into_boxed_str());
                media_type
            })
            .collect::<Vec<&str>>();

        let file_format = FileFormat {
            id: language.id,
            source_type: SourceType::Linguist,
            name: Box::leak(language.name.into_boxed_str()),
            extensions: Box::leak(extensions.into_boxed_slice()),
            media_types: Box::leak(media_types.into_boxed_slice()),
            ..Default::default()
        };
        file_formats.push(file_format);
    }

    file_formats
}

async fn generate_source_code(
    file_formats: &Vec<FileFormat>,
    source_dir: &Path,
    dry_run: bool,
) -> Result<()> {
    // Generate the module file
    let mut source_code = vec!["use crate::format::FileFormat;".to_string(), String::new()];
    for file_format in file_formats {
        let name = format!("linguist_{}", file_format.id);
        source_code.push(format!("mod {name};"));
    }
    source_code.push(String::new());
    source_code.push("pub(crate) const FILE_FORMATS: &[&FileFormat] = &[".to_string());
    for file_format in file_formats {
        let name = format!("linguist_{}", file_format.id);
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
        let name = format!("linguist_{}", file_format.id);
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

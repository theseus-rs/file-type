#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
use file_type::format::{FileFormat, SourceType};
use reqwest::Client;
use serde_json::Value;
use source_generator::generate;
use std::env;
use std::path::PathBuf;
use std::time::Duration;
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
    generate(&file_formats, &source_dir, dry_run)?;
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

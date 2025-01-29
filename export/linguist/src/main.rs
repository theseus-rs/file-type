#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
use file_type::format::{DocumentIdentifier, ExternalSignature, FileFormat, SignatureType};
use quick_xml::se::Serializer;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::info;
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
        .with_env_var("LINGUIST_LOG")
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
        .join("linguist");
    fs::create_dir_all(&new_dir).await?;

    let client = Client::new();
    let response = client
        .get(MIME_TYPES_URL)
        .header("User-Agent", format!("{CRATE_NAME}/{CRATE_VERSION}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await?;

    let language_data = response.text().await?;
    let mime_types = parse_languages(language_data)?;
    let file_formats = process_languages(mime_types);
    for file_format in file_formats {
        store_file_format(&file_format, &new_dir).await?;
    }

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
        let extensions = match language.get("extensions") {
            Some(Value::Array(extensions)) => extensions
                .iter()
                .map(|extension| {
                    let extension = extension.as_str().expect("Invalid extension").to_string();
                    extension.trim_start_matches('.').to_string()
                })
                .collect(),
            _ => Vec::new(),
        };

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
        let puid = format!("linguist/{}", language.id);
        let mime_type = &language.mime_type;
        let extensions = &language.extensions;
        let mut file_format_identifiers = vec![DocumentIdentifier::new(puid, "PUID".to_string())];

        if !mime_type.is_empty() {
            file_format_identifiers.push(DocumentIdentifier::new(mime_type.as_str(), "MIME"));
        }

        let external_signatures = extensions
            .iter()
            .enumerate()
            .map(|(index, extension)| {
                ExternalSignature::new(index, extension.as_str(), SignatureType::FileExtension)
            })
            .collect::<Vec<ExternalSignature>>();

        let file_format = FileFormat::new(
            language.id,
            language.name.as_str(),
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            file_format_identifiers,
            external_signatures,
            vec![],
            vec![],
            vec![],
        );
        file_formats.push(file_format);
    }

    file_formats
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

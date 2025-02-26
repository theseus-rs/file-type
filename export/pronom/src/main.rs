#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

mod format;

use crate::format::pronom_report::PronomReport;
use anyhow::Result;
use file_type::format::FileFormat;
use quick_xml::de;
use rayon::prelude::*;
use reqwest::blocking::Client;
use source_generator::generate;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const BASE_URL: &str = "https://www.nationalarchives.gov.uk/PRONOM/";

/// Downloads PRONOM data and saves it to the `file_type` crate data directory.
fn main() -> Result<()> {
    initialize_tracing();

    let mut max_fmt_puid = 2039;
    if let Ok(fmt_puid) = env::var("MAX_FMT_PUID") {
        if let Ok(fmt_puid) = fmt_puid.parse::<i64>() {
            max_fmt_puid = fmt_puid;
        }
    }

    let mut max_x_fmt_puid = 455;
    if let Ok(x_fmt_puid) = env::var("MAX_X_FMT_PUID") {
        if let Ok(x_fmt_puid) = x_fmt_puid.parse::<i64>() {
            max_x_fmt_puid = x_fmt_puid;
        }
    }

    let puid_ids = HashMap::from([("fmt", max_fmt_puid), ("x-fmt", max_x_fmt_puid)]);
    let dry_run = env::var("DRY_RUN").is_ok();
    execute(puid_ids, dry_run)?;
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
        .with_env_var("PRONOM_EXPORT_LOG")
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .fmt_fields(fmt::format::DefaultFields::new())
        .event_format(format)
        .init();
}

fn execute(puid_types: HashMap<&str, i64>, dry_run: bool) -> Result<()> {
    let source_dir = PathBuf::from(CRATE_DIR)
        .join("..")
        .join("..")
        .join("file_type")
        .join("src")
        .join("sources")
        .join("pronom");
    let client = Client::new();
    let mut file_formats = Vec::new();

    for (puid_type, puid_range) in puid_types {
        let puid_type_url = format!("{BASE_URL}{puid_type}/");

        let tasks = (1..=puid_range)
            .map(|index| format!("{puid_type_url}{index}.xml"))
            .collect::<Vec<String>>();

        let formats = tasks
            .par_iter()
            .filter_map(|puid_url| match download(&client, puid_url) {
                Ok(file_format) => Some(file_format),
                Err(error) => {
                    error!("Failed to download and save {puid_url}: {error}");
                    None
                }
            })
            .collect::<Vec<FileFormat>>();
        file_formats.extend_from_slice(&formats);
    }

    file_formats.sort_by_key(|file_format| file_format.id);
    generate(&file_formats, &source_dir, dry_run)?;
    Ok(())
}

fn download(client: &Client, puid_url: &String) -> Result<FileFormat> {
    info!("{puid_url}");
    let response = client
        .get(puid_url)
        .header("Accept", "text/xml")
        .header("User-Agent", format!("{CRATE_NAME}/{CRATE_VERSION}"))
        .timeout(Duration::from_secs(30))
        .send()?;
    let xml = response.text()?;
    let pronom_report: PronomReport = de::from_str(&xml)?;
    let pronom_file_format = pronom_report.detail.file_format;
    let file_format = pronom_file_format.to_runtime_type()?;
    Ok(file_format)
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

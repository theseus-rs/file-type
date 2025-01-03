#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

mod pronom_report;
mod report_format_detail;

use crate::pronom_report::PronomReport;
use anyhow::Result;
use quick_xml::de::from_str;
use quick_xml::se::Serializer;
use rayon::prelude::*;
use reqwest::blocking::Client;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tracing::{error, info, warn};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, EnvFilter};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const BASE_URL: &str = "https://www.nationalarchives.gov.uk/PRONOM/";

/// Downloads PRONOM data and saves it to the `file_type` crate data directory.
fn main() -> Result<()> {
    initialize_tracing();

    let mut max_fmt_puid = 2009;
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

    export_data(puid_ids)?;
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

fn export_data(puid_types: HashMap<&str, i64>) -> Result<()> {
    let client = Client::new();

    for (puid_type, puid_range) in puid_types {
        let puid_type_url = format!("{BASE_URL}{puid_type}/");
        let new_dir = PathBuf::from(CRATE_DIR)
            .join("..")
            .join("..")
            .join("file_type")
            .join("data")
            .join("pronom");
        fs::create_dir_all(&new_dir)?;

        let tasks: Vec<_> = (1..=puid_range)
            .map(|index| {
                let puid_url = format!("{puid_type_url}{index}.xml");
                let file_name = new_dir.join(format!("{puid_type}-{index}.xml"));
                (puid_url, file_name)
            })
            .collect();

        tasks.par_iter().for_each(|(puid_url, file_name)| {
            download_and_save_puid(&client, puid_url, file_name).unwrap_or_else(|error| {
                error!("Failed to download and save {puid_url}: {error}");
            });
        });
    }

    Ok(())
}

fn download_and_save_puid(client: &Client, puid_url: &String, file_name: &PathBuf) -> Result<()> {
    info!("{puid_url}");

    let response = client
        .get(puid_url)
        .header("Accept", "text/xml")
        .header("User-Agent", format!("{CRATE_NAME}/{CRATE_VERSION}"))
        .timeout(Duration::from_secs(30))
        .send()?;
    let xml = response.text()?;

    let pronom_report: PronomReport = match from_str(&xml) {
        Ok(report) => report,
        Err(error) => {
            warn!(
                "Not writing record {}: ({error})",
                file_name.to_string_lossy()
            );
            return Ok(());
        }
    };

    let detail = pronom_report.detail();
    let file_format = detail.file_format();
    let mut buffer = String::new();
    let mut serializer = Serializer::new(&mut buffer);
    serializer.indent(' ', 2);
    file_format.serialize(serializer)?;
    File::create(file_name).and_then(|mut file| file.write_all(buffer.as_bytes()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        env::set_var("MAX_FMT_PUID", "1");
        env::set_var("MAX_X_FMT_PUID", "1");
        let result = main();
        assert!(result.is_ok());
    }
}

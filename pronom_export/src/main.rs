#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

mod pronom_report;
mod report_format_detail;

use crate::pronom_report::PronomReport;
use quick_xml::de::from_str;
use quick_xml::se::Serializer;
use rayon::prelude::*;
use reqwest::blocking::Client;
use serde::Serialize;
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
const PUID_TYPES: [(&str, i64); 2] = [("fmt", 2009), ("x-fmt", 455)];

/// Downloads PRONOM data and saves it to the data directory.
fn main() {
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

    export_data();
}

fn export_data() {
    let client = Client::new();

    for (puid_type, puid_range) in PUID_TYPES {
        let puid_type_url = format!("{BASE_URL}{puid_type}/");
        let new_dir = PathBuf::from(CRATE_DIR)
            .join("..")
            .join("file_type")
            .join("data")
            .join("pronom");

        if let Err(e) = fs::create_dir_all(&new_dir) {
            error!(
                "Failed to create directory {}: {e}",
                new_dir.to_string_lossy()
            );
            continue;
        }

        let tasks: Vec<_> = (1..=puid_range)
            .map(|index| {
                let puid_url = format!("{puid_type_url}{index}.xml");
                let file_name = new_dir.join(format!("{puid_type}-{index}.xml"));
                (puid_url, file_name)
            })
            .collect();

        tasks.par_iter().for_each(|(puid_url, file_name)| {
            download_and_save_puid(&client, puid_url, file_name);
        });
    }
}

fn download_and_save_puid(client: &Client, puid_url: &String, file_name: &PathBuf) {
    info!("{puid_url}");

    let response = match client
        .get(puid_url)
        .header("Accept", "text/xml")
        .header("User-Agent", format!("{CRATE_NAME}/{CRATE_VERSION}"))
        .timeout(Duration::from_secs(30))
        .send()
    {
        Ok(response) => response,
        Err(error) => {
            error!("Failed to download {puid_url}: {error}");
            return;
        }
    };

    let xml = match response.text() {
        Ok(text) => text,
        Err(error) => {
            error!("Failed to get text from response {puid_url}: {error}");
            return;
        }
    };

    let pronom_report: PronomReport = match from_str(&xml) {
        Ok(report) => report,
        Err(error) => {
            warn!(
                "Not writing record {}: ({error})",
                file_name.to_string_lossy()
            );
            return;
        }
    };

    let detail = pronom_report.detail();
    let file_format = detail.file_format();
    let mut buffer = String::new();
    let mut serializer = Serializer::new(&mut buffer);
    serializer.indent(' ', 2);

    let xml = match file_format.serialize(serializer) {
        Ok(_) => buffer,
        Err(error) => {
            error!(
                "Failed to serialize file format {}: {error}",
                file_name.to_string_lossy()
            );
            return;
        }
    };

    if let Err(error) = File::create(file_name).and_then(|mut file| file.write_all(xml.as_bytes()))
    {
        error!(
            "Failed to write file {}: {error}",
            file_name.to_string_lossy()
        );
    }
}

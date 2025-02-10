#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
use file_type::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType};
use reqwest::blocking::Client;
use serde_json::Value;
use source_generator::generate;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use tracing::warn;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, EnvFilter};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const SPARQL_URL: &str = "https://query.wikidata.org/sparql?query=%23List%20of%20computer%20file%20formats%20with%20all%20identification%20patterns%0ASELECT%20DISTINCT%20%3FidExtension%20%3Fextension%20%3FmediaType%20%3FpronomId%20%3FidExtensionLabel%20%3FfileSignature%0AWHERE%0A%7B%0A%20%20%23%20Find%20instances%20of%20file%20format%20(Q235557)%0A%20%20%3FidExtension%20wdt%3AP31%20wd%3AQ235557%20%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%23%20Get%20the%20file%20extension%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20wdt%3AP1195%20%3Fextension%20.%0A%20%20%0A%20%20%23%20Optional%3A%20Get%20the%20MIME%2Fmedia%20type%20if%20available%0A%20%20OPTIONAL%20%7B%20%3FidExtension%20wdt%3AP1163%20%3FmediaType%20%7D%0A%20%20%0A%20%20%23%20Optional%3A%20Get%20the%20PRONOM%20ID%20if%20available%0A%20%20OPTIONAL%20%7B%20%3FidExtension%20wdt%3AP2748%20%3FpronomId%20%7D%0A%20%20%0A%20%20%23%20Optional%3A%20Get%20file%20signatures%2Fmagic%20numbers%20if%20available%0A%20%20OPTIONAL%20%7B%20%3FidExtension%20wdt%3AP4152%20%3FfileSignature%20%7D%0A%20%20%0A%20%20%23%20Optional%3A%20Get%20byte%20sequences%20if%20available%0A%20%20OPTIONAL%20%7B%20%3FidExtension%20wdt%3AP5933%20%3FbyteSequence%20%7D%0A%20%20%0A%20%20%23%20Optional%3A%20Get%20offset%20patterns%20if%20available%0A%20%20OPTIONAL%20%7B%20%0A%20%20%20%20%3FidExtension%20p%3AP5933%20%3Fstatement%20.%0A%20%20%20%20%3Fstatement%20ps%3AP5933%20%3FoffsetPattern%20%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20pq%3AP4153%20%3Foffset%20.%0A%20%20%7D%0A%20%20%0A%20%20%23%20Optional%3A%20Get%20magic%20numbers%20if%20available%20(different%20from%20file%20signatures)%0A%20%20OPTIONAL%20%7B%20%3FidExtension%20wdt%3AP3508%20%3FmagicNumber%20%7D%0A%20%20%0A%20%20%23%20Optional%3A%20Get%20format%20identification%20notes%20if%20available%0A%20%20OPTIONAL%20%7B%20%3FidExtension%20wdt%3AP2561%20%3FidentificationNote%20%7D%0A%20%20%0A%20%20%23%20Add%20labels%20in%20English%0A%20%20SERVICE%20wikibase%3Alabel%20%7B%20bd%3AserviceParam%20wikibase%3Alanguage%20%22en%22%20%7D%0A%7D%0AGROUP%20BY%20%3FidExtension%20%3Fextension%20%3FmediaType%20%3FpronomId%20%3FidExtensionLabel%20%3FfileSignature%0AORDER%20BY%20%3Fextension%20%3FmediaType";

// SPARQL
//
// SELECT DISTINCT ?idExtension ?extension ?mediaType ?pronomId ?idExtensionLabel ?fileSignature
// WHERE
// {
//   # Find instances of file format (Q235557)
//   ?idExtension wdt:P31 wd:Q235557 ;
//                # Get the file extension
//                wdt:P1195 ?extension .
//
//   # Optional: Get the MIME/media type if available
//   OPTIONAL { ?idExtension wdt:P1163 ?mediaType }
//
//   # Optional: Get the PRONOM ID if available
//   OPTIONAL { ?idExtension wdt:P2748 ?pronomId }
//
//   # Optional: Get file signatures/magic numbers if available
//   OPTIONAL { ?idExtension wdt:P4152 ?fileSignature }
//
//   # Optional: Get byte sequences if available
//   OPTIONAL { ?idExtension wdt:P5933 ?byteSequence }
//
//   # Optional: Get offset patterns if available
//   OPTIONAL {
//     ?idExtension p:P5933 ?statement .
//     ?statement ps:P5933 ?offsetPattern ;
//               pq:P4153 ?offset .
//   }
//
//   # Optional: Get magic numbers if available (different from file signatures)
//   OPTIONAL { ?idExtension wdt:P3508 ?magicNumber }
//
//   # Optional: Get format identification notes if available
//   OPTIONAL { ?idExtension wdt:P2561 ?identificationNote }
//
//   # Add labels in English
//   SERVICE wikibase:label { bd:serviceParam wikibase:language "en" }
// }
// GROUP BY ?idExtension ?extension ?mediaType ?pronomId ?idExtensionLabel ?fileSignature
// ORDER BY ?extension ?mediaType

/// Downloads Wikidata file format data and saves it to the `file_type` crate data directory.
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
        .with_env_var("WIKIDATA_LOG")
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
        .join("wikidata");

    let client = Client::new();
    let response = client
        .get(SPARQL_URL)
        .header("Accept", "application/json")
        .header("User-Agent", format!("{CRATE_NAME}/{CRATE_VERSION}"))
        .timeout(Duration::from_secs(30))
        .send()?;

    let json: Value = response.json()?;
    let mut file_formats = parse_json(&json);
    file_formats.sort_by_key(|file_format| file_format.id);
    generate(&file_formats, &source_dir, dry_run)?;
    Ok(())
}

fn parse_json(json: &Value) -> Vec<FileFormat> {
    let mut file_formats: HashMap<usize, FileFormat> = HashMap::new();

    let Some(bindings) = json
        .get("results")
        .and_then(|results| results.get("bindings"))
        .and_then(|bindings| bindings.as_array())
    else {
        warn!("no results found");
        return Vec::new();
    };
    for binding in bindings {
        let id_extension = binding
            .get("idExtension")
            .and_then(|id_extension| id_extension.get("value"))
            .and_then(|id_extension| id_extension.as_str())
            .unwrap_or_default();

        let mut parts = id_extension.rsplitn(2, "/Q");
        let id = parts
            .next()
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or_default();

        let mut extensions = Vec::new();
        let mut media_types = Vec::new();
        let mut internal_signatures = Vec::new();

        if let Some(file_format) = file_formats.get(&id) {
            extensions = file_format.extensions.to_vec();
            media_types = file_format.media_types.to_vec();
            internal_signatures = file_format.signatures.to_vec();
        }

        let name = binding
            .get("idExtensionLabel")
            .and_then(|id_extension_label| id_extension_label.get("value"))
            .and_then(|id_extension_label| id_extension_label.as_str())
            .unwrap_or_default();
        let name = Box::leak(name.to_string().into_boxed_str());

        if let Some(extension) = binding
            .get("extension")
            .and_then(|extension| extension.get("value"))
            .and_then(|extension| extension.as_str())
        {
            if !extensions.contains(&extension) {
                let extension = Box::leak(extension.to_string().into_boxed_str());
                extensions.push(extension);
                extensions.sort_unstable();
            }
        };

        if let Some(media_type) = binding
            .get("mediaType")
            .and_then(|media_type| media_type.get("value"))
            .and_then(|media_type| media_type.as_str())
        {
            if !media_types.contains(&media_type) {
                let media_type = Box::leak(media_type.to_string().into_boxed_str());
                media_types.push(media_type);
                media_types.sort_unstable();
            }
        }

        if let Some(file_signature) = binding
            .get("fileSignature")
            .and_then(|file_signature| file_signature.get("value"))
            .and_then(|file_signature| file_signature.as_str())
        {
            let file_signature = file_signature.replace(' ', "");

            if let Ok(regex) = Regex::new(file_signature.as_str()) {
                let byte_sequence = ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex,
                };
                let internal_signature = Signature {
                    byte_sequences: Box::leak(vec![byte_sequence].into_boxed_slice()),
                };

                if !internal_signatures.contains(&internal_signature) {
                    internal_signatures.push(internal_signature);
                }
            } else {
                warn!("id {id}; invalid regex: {file_signature}");
            }
        }

        let file_format = FileFormat {
            id,
            source_type: SourceType::Wikidata,
            name,
            extensions: Box::leak(extensions.into_boxed_slice()),
            media_types: Box::leak(media_types.into_boxed_slice()),
            signatures: Box::leak(internal_signatures.into_boxed_slice()),
            related_formats: &[],
        };
        file_formats.insert(id, file_format);
    }

    let file_formats: Vec<FileFormat> = file_formats.values().cloned().collect();
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

#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
use file_type::format::{
    ByteSequence, DocumentIdentifier, ExternalSignature, FileFormat, InternalSignature,
    PositionType, Regex, SignatureType,
};
use quick_xml::se::Serializer;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{info, warn};
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

/// Downloads Wikidata file format data and saves it to the file_type crate data directory.
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
        .with_env_var("WIKIDATA_LOG")
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
        .join("wikidata");
    fs::create_dir_all(&new_dir).await?;

    let client = Client::new();
    let response = client
        .get(SPARQL_URL)
        .header("Accept", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", format!("{CRATE_NAME}/{CRATE_VERSION}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await?;

    let json: Value = response.json().await?;
    let file_formats = parse_json(&json);
    for (_puid, file_format) in file_formats {
        store_file_format(&file_format, &new_dir).await?;
    }

    Ok(())
}

fn parse_json(json: &Value) -> HashMap<String, FileFormat> {
    let mut file_formats: HashMap<String, FileFormat> = HashMap::new();

    let Some(bindings) = json
        .get("results")
        .and_then(|results| results.get("bindings"))
        .and_then(|bindings| bindings.as_array())
    else {
        warn!("no results found");
        return file_formats;
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
        let puid = format!("wikidata/{id}");

        let mut file_format_identifiers = Vec::new();
        let mut external_signatures = Vec::new();
        let mut internal_signatures = Vec::new();

        if let Some(file_format) = file_formats.get(&puid) {
            file_format_identifiers = file_format.file_format_identifiers().to_vec();
            external_signatures = file_format.external_signatures().to_vec();
            internal_signatures = file_format.internal_signatures().to_vec();
        } else {
            file_format_identifiers.push(DocumentIdentifier::new(puid.as_str(), "PUID"));
        }

        if let Some(media_type) = binding
            .get("mediaType")
            .and_then(|media_type| media_type.get("value"))
            .and_then(|media_type| media_type.as_str())
        {
            file_format_identifiers.push(DocumentIdentifier::new(media_type, "MIME"));
        }

        let name = binding
            .get("idExtensionLabel")
            .and_then(|id_extension_label| id_extension_label.get("value"))
            .and_then(|id_extension_label| id_extension_label.as_str())
            .unwrap_or_default();

        if let Some(extension) = binding
            .get("extension")
            .and_then(|extension| extension.get("value"))
            .and_then(|extension| extension.as_str())
        {
            external_signatures.push(ExternalSignature::new(
                0,
                extension,
                SignatureType::FileExtension,
            ));
        };

        if let Some(file_signature) = binding
            .get("fileSignature")
            .and_then(|file_signature| file_signature.get("value"))
            .and_then(|file_signature| file_signature.as_str())
        {
            let file_signature = file_signature.replace(' ', "");

            if file_signature.len() < 4 {
                warn!("id {id}; file signature to short: {file_signature}");
            } else if let Ok(regex) = Regex::new(file_signature.as_str()) {
                let byte_sequence = ByteSequence::new(
                    0,
                    PositionType::AbsoluteFromBOF,
                    Some(0),
                    None, // max_offset
                    None, // indirect_offset_location
                    None, // indirect_offset_length
                    None, // endianness
                    regex,
                );
                let internal_signature = InternalSignature::new(0, name, "", vec![byte_sequence]);
                internal_signatures.push(internal_signature);
            } else {
                warn!("id {id}; invalid regex: {file_signature}");
            }
        }

        let file_format = FileFormat::new(
            id,
            name,
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
            internal_signatures,
            vec![],
            vec![],
        );
        file_formats.insert(puid, file_format);
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

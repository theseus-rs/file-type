use anyhow::Result;
use file_type::format::PositionType;
use file_type::sources::wikidata;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub(crate) fn generate() -> Result<()> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let wikidata = PathBuf::from(crate_dir).join("wikidata");
    for file_format in wikidata::FILE_FORMATS.iter().copied() {
        let id = file_format.id;

        for extension in file_format.extensions {
            // Skip if the extension contains invalid characters
            if extension.contains(|c: char| ['?', '*', ':'].contains(&c)) {
                eprintln!("Invalid extension: {extension}");
                continue;
            }

            let path = wikidata.join(format!("wikidata-{id}.{extension}"));
            if File::create(&path).is_err() {
                eprintln!("Failed to create extension file: {path:?}");
            }
        }

        for (index, signature) in file_format.signatures.iter().enumerate() {
            let path = wikidata.join(format!("wikidata-{id}.signature_{index}"));
            let test_data_signature = if let Some(byte_sequence) = signature.byte_sequences.first()
            {
                if matches!(byte_sequence.position_type, PositionType::BOF)
                    && byte_sequence.offset == Some(0)
                {
                    byte_sequence.regex.test_data()
                } else {
                    continue;
                }
            } else {
                Vec::new()
            };
            match File::create(&path) {
                Ok(mut file) => {
                    file.write_all(&test_data_signature)?;
                }
                Err(_) => {
                    eprintln!("Failed to create signature file: {path:?}");
                }
            }
        }
    }
    Ok(())
}

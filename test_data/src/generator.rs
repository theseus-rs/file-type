use crate::test_signature::TestSignature;
use anyhow::Result;
use file_type::FileType;
use file_type::format::SourceType;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub(crate) fn generate(source_type: &SourceType, file_types: &[&FileType]) -> Result<()> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let source_type_name = format!("{source_type:?}").to_lowercase();
    let test_data_path = PathBuf::from(crate_dir).join(&source_type_name);
    for file_type in file_types {
        let id = file_type.id();

        for extension in file_type.extensions() {
            // Skip if the extension contains invalid characters
            if !is_valid_extension(extension) {
                eprintln!("Invalid extension: {extension}");
                continue;
            }

            let path = test_data_path.join(format!("{source_type_name}-{id}.{extension}"));
            if File::create(&path).is_err() {
                eprintln!("Failed to create extension file: {path:?}");
            }
        }

        let extension = match file_type.extensions().first() {
            Some(extension) if is_valid_extension(extension) => format!(".{extension}"),
            _ => String::new(),
        };

        for (index, signature) in file_type.file_format().signatures.iter().enumerate() {
            let path = test_data_path.join(format!(
                "{source_type_name}-{id}-signature-{index}{extension}"
            ));
            let test_data_signature = signature.to_test_signature();
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

fn is_valid_extension(extension: &str) -> bool {
    !extension.contains(|c: char| ['?', '*', ':'].contains(&c))
}

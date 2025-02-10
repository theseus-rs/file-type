use crate::Source;
use anyhow::Result;
use file_type::format::FileFormat;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tracing::{info, warn};

pub fn generate(file_formats: &Vec<FileFormat>, source_dir: &Path, dry_run: bool) -> Result<()> {
    let first_file_format = file_formats.first().expect("No file formats");
    let source = format!("{:?}", first_file_format.source_type).to_lowercase();

    // Generate the module file
    let mut source_code = vec!["use crate::FileType;".to_string(), String::new()];
    for file_format in file_formats {
        let name = format!("{source}_{}", file_format.id);
        source_code.push(format!("mod {name};"));
    }
    source_code.push(String::new());
    source_code.push("#[doc(hidden)]".to_string());
    source_code.push("pub const FILE_TYPES: &[&FileType] = &[".to_string());
    for file_format in file_formats {
        let name = format!("{source}_{}", file_format.id);
        source_code.push(format!("    &{name}::{},", name.to_uppercase()));
    }
    source_code.push("];".to_string());
    source_code.push(String::new());
    let file_name = source_dir.join("mod.rs");
    if dry_run {
        warn!("[dry-run] Writing {}", file_name.to_string_lossy());
    } else {
        info!("Writing {}", file_name.to_string_lossy());
        let mut source_file = File::create(file_name)?;
        source_file.write_all(source_code.join("\n").as_bytes())?;
    }

    // Generate source files for each file format
    for file_format in file_formats {
        let mut structs = vec!["FileFormat", "SourceType"];
        if !file_format.signatures.is_empty() {
            structs.extend_from_slice(&[
                "ByteSequence",
                "PositionType",
                "Regex",
                "Signature",
                "Token",
            ]);
        }
        if !file_format.related_formats.is_empty() {
            structs.extend_from_slice(&["RelatedFormat", "RelationshipType"]);
        }
        structs.sort();
        let name = format!("{source}_{}", file_format.id);
        let source_code = [
            format!(
                "use crate::format::{{{structs}}};",
                structs = structs.join(", ")
            ),
            "use crate::FileType;".to_string(),
            String::new(),
            format!(
                "pub(crate) const {}: FileType = FileType {{ file_format: &{} }};",
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
            let mut source_file = File::create(file_name)?;
            source_file.write_all(source_code.as_bytes())?;
        }
    }
    Ok(())
}

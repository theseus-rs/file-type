use anyhow::{Result, bail};
use file_type::format::{FileFormat, Signature, SourceType};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tracing::{info, warn};

/// Generate the lookup maps used by the `file_type` crate.
///
/// # Errors
/// Returns an error when no file formats are supplied or an output file cannot be written.
pub fn generate_maps(file_formats: &[&FileFormat], source_dir: &Path, dry_run: bool) -> Result<()> {
    if file_formats.is_empty() {
        bail!("No file formats");
    }

    generate_signatures_map(file_formats, source_dir, dry_run)?;
    generate_extension_map(file_formats, source_dir, dry_run)?;
    generate_media_types_map(file_formats, source_dir, dry_run)
}

fn generate_signatures_map(
    file_formats: &[&FileFormat],
    source_dir: &Path,
    dry_run: bool,
) -> Result<()> {
    let mut signatures = BTreeMap::new();
    for file_format in file_formats {
        for key in file_format.signatures.iter().map(Signature::key) {
            let references: &mut Vec<&FileFormat> = signatures.entry(key).or_default();
            references.push(file_format);
            references.sort();
        }
    }

    let mut map = phf_codegen::Map::new();
    for (signature, file_formats) in signatures {
        let references = references_source(&file_formats);
        map.entry(signature, references);
    }

    let imports = [
        "use crate::FileType;",
        "#[cfg(feature = \"wikidata\")]",
        "use crate::sources::wikidata;",
    ];
    let source = format!(
        "{}\n\n#[expect(clippy::unreadable_literal)]\npub(crate) static MAP: phf::Map<u64, &'static [&'static FileType]> = {};\n",
        imports.join("\n"),
        map.build()
    );
    write_source(&source_dir.join("signatures.rs"), &source, dry_run)
}

fn generate_extension_map(
    file_formats: &[&FileFormat],
    source_dir: &Path,
    dry_run: bool,
) -> Result<()> {
    let mut extensions = BTreeMap::new();
    for file_format in file_formats {
        for extension in file_format.extensions {
            let references: &mut Vec<&FileFormat> = extensions.entry(*extension).or_default();
            references.push(file_format);
            references.sort();
        }
    }

    let mut map = phf_codegen::Map::new();
    for (extension, file_formats) in extensions {
        let references = references_source(&file_formats);
        map.entry(extension, references);
    }

    let imports = [
        "use crate::FileType;",
        "#[cfg(feature = \"httpd\")]",
        "use crate::sources::httpd;",
        "#[cfg(feature = \"linguist\")]",
        "use crate::sources::linguist;",
        "#[cfg(feature = \"wikidata\")]",
        "use crate::sources::wikidata;",
    ];
    let source = format!(
        "{}\n\n#[expect(clippy::unreadable_literal)]\npub(crate) static MAP: phf::Map<&'static str, &'static [&'static FileType]> = {};\n",
        imports.join("\n"),
        map.build()
    );
    write_source(&source_dir.join("extensions.rs"), &source, dry_run)
}

fn generate_media_types_map(
    file_formats: &[&FileFormat],
    source_dir: &Path,
    dry_run: bool,
) -> Result<()> {
    let mut media_types = BTreeMap::new();
    for file_format in file_formats {
        for media_type in file_format.media_types {
            let references: &mut Vec<&FileFormat> = media_types.entry(*media_type).or_default();
            references.push(file_format);
            references.sort();
        }
    }

    let mut map = phf_codegen::Map::new();
    for (media_type, file_formats) in media_types {
        let references = references_source(&file_formats);
        map.entry(media_type, references);
    }

    let imports = [
        "use crate::FileType;",
        "use crate::sources::default;",
        "#[cfg(feature = \"httpd\")]",
        "use crate::sources::httpd;",
        "#[cfg(feature = \"iana\")]",
        "use crate::sources::iana;",
        "#[cfg(feature = \"linguist\")]",
        "use crate::sources::linguist;",
        "#[cfg(feature = \"wikidata\")]",
        "use crate::sources::wikidata;",
    ];
    let source = format!(
        "{}\n\n#[expect(clippy::unreadable_literal)]\npub(crate) static MAP: phf::Map<&'static str, &'static [&'static FileType]> = {};\n",
        imports.join("\n"),
        map.build()
    );
    write_source(&source_dir.join("media_types.rs"), &source, dry_run)
}

fn references_source(file_formats: &[&FileFormat]) -> String {
    let references = file_formats
        .iter()
        .map(|file_format| file_format_reference(file_format))
        .collect::<Vec<String>>();
    format!("&[{}]", references.join(","))
}

fn file_format_reference(file_format: &FileFormat) -> String {
    let source = format!("{:?}", file_format.source_type).to_lowercase();
    let id = file_format.id;
    let constant = format!("{}_{id}", source.to_uppercase());
    let reference = format!("&{source}::{constant}");

    if matches!(file_format.source_type, SourceType::Default) {
        reference
    } else {
        format!("\n#[cfg(feature = \"{source}\")]\n{reference}")
    }
}

fn write_source(path: &Path, source: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        warn!("[dry-run] Writing {}", path.to_string_lossy());
        return Ok(());
    }

    info!("Writing {}", path.to_string_lossy());
    let file = File::create(path)?;
    let mut file = BufWriter::new(file);
    file.write_all(source.as_bytes())?;
    Ok(())
}

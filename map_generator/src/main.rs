//! # Generate Maps
//!
//! This crate generates the maps used by the `file_type` module to reduce build times and increase
//! runtime performance.

#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
use file_type::format::{Signature, SourceType};
use file_type::sources::file_types;
use file_type::FileType;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let crate_dir = env::var("CARGO_MANIFEST_DIR")?;
    let source_dir = PathBuf::from(crate_dir)
        .join("..")
        .join("file_type")
        .join("src");

    generate_signatures_map(&source_dir)?;
    generate_extension_map(&source_dir)?;
    generate_media_types_map(&source_dir)?;
    Ok(())
}

/// Generate the signatures map source file
fn generate_signatures_map(source_dir: &Path) -> Result<()> {
    let mut signatures = HashMap::new();
    for file_type in file_types() {
        let file_format = file_type.file_format();
        let signature_keys = file_format
            .signatures
            .iter()
            .map(Signature::key)
            .collect::<Vec<u64>>();
        for key in signature_keys {
            let mut file_types: Vec<&FileType> = signatures.remove(&key).unwrap_or_default();
            file_types.push(file_type);
            file_types.sort();
            signatures.insert(key, file_types);
        }
    }

    let mut map = &mut phf_codegen::Map::new();
    for (signature, references) in signatures {
        let references = references
            .iter()
            .copied()
            .map(file_type_reference)
            .collect::<Vec<String>>();
        let references = format!("&[{}]", references.join(", "));
        map = map.entry(signature, references.as_str());
    }

    let path = source_dir.join("signatures.rs");
    let file = File::create(&path)?;
    let mut file = BufWriter::new(file);
    let use_statements = [
        "use crate::FileType;",
        "#[cfg(feature = \"custom\")]",
        "use crate::sources::custom;",
        "#[cfg(feature = \"pronom\")]",
        "use crate::sources::pronom;",
        "#[cfg(feature = \"wikidata\")]",
        "use crate::sources::wikidata;",
    ];
    writeln!(&mut file, "{}\n", use_statements.join("\n"))?;

    writeln!(&mut file, "#[expect(clippy::unreadable_literal)]")?;
    write!(
        &mut file,
        "pub(crate) static MAP: phf::Map<u64, &'static [&'static FileType]> = {}",
        map.build()
    )?;
    writeln!(&mut file, ";")?;
    Ok(())
}

/// Generate the extension map source file
fn generate_extension_map(source_dir: &Path) -> Result<()> {
    let mut extensions = HashMap::new();
    for file_type in file_types() {
        for extension in file_type.extensions() {
            let extension = *extension;
            let mut file_types = extensions.get(extension).unwrap_or(&vec![]).clone();
            file_types.push(file_type);
            file_types.sort();
            extensions.insert(extension, file_types);
        }
    }

    let mut map = &mut phf_codegen::Map::new();
    for (extension, references) in extensions {
        let references = references
            .iter()
            .copied()
            .map(file_type_reference)
            .collect::<Vec<String>>();
        let references = format!("&[{}]", references.join(", "));
        map = map.entry(extension, references.as_str());
    }

    let path = source_dir.join("extensions.rs");
    let file = File::create(&path)?;
    let mut file = BufWriter::new(file);
    let use_statements = [
        "use crate::FileType;",
        "#[cfg(feature = \"custom\")]",
        "use crate::sources::custom;",
        "#[cfg(feature = \"httpd\")]",
        "use crate::sources::httpd;",
        "#[cfg(feature = \"linguist\")]",
        "use crate::sources::linguist;",
        "#[cfg(feature = \"pronom\")]",
        "use crate::sources::pronom;",
        "#[cfg(feature = \"wikidata\")]",
        "use crate::sources::wikidata;",
    ];
    writeln!(&mut file, "{}\n", use_statements.join("\n"))?;

    writeln!(&mut file, "#[expect(clippy::unreadable_literal)]")?;
    write!(
        &mut file,
        "pub(crate) static MAP: phf::Map<&'static str, &'static [&'static FileType]> = {}",
        map.build()
    )?;
    writeln!(&mut file, ";")?;
    Ok(())
}

/// Generate the media types map source file
fn generate_media_types_map(source_dir: &Path) -> Result<()> {
    let mut media_types = HashMap::new();
    for file_type in file_types() {
        for media_type in file_type.media_types() {
            let media_type = *media_type;
            let mut file_types = media_types.get(media_type).unwrap_or(&vec![]).clone();
            file_types.push(file_type);
            file_types.sort();
            media_types.insert(media_type, file_types);
        }
    }

    let mut map = &mut phf_codegen::Map::new();
    for (media_type, references) in media_types {
        let references = references
            .iter()
            .copied()
            .map(file_type_reference)
            .collect::<Vec<String>>();
        let references = format!("&[{}]", references.join(", "));
        map = map.entry(media_type, references.as_str());
    }

    let path = source_dir.join("media_types.rs");
    let file = File::create(&path)?;
    let mut file = BufWriter::new(file);
    let use_statements = vec![
        "use crate::FileType;",
        "#[cfg(feature = \"custom\")]",
        "use crate::sources::custom;",
        "use crate::sources::default;",
        "#[cfg(feature = \"httpd\")]",
        "use crate::sources::httpd;",
        "#[cfg(feature = \"iana\")]",
        "use crate::sources::iana;",
        "#[cfg(feature = \"linguist\")]",
        "use crate::sources::linguist;",
        "#[cfg(feature = \"pronom\")]",
        "use crate::sources::pronom;",
        "#[cfg(feature = \"wikidata\")]",
        "use crate::sources::wikidata;",
    ];
    writeln!(&mut file, "{}\n", use_statements.join("\n"))?;

    writeln!(&mut file, "#[expect(clippy::unreadable_literal)]")?;
    write!(
        &mut file,
        "pub(crate) static MAP: phf::Map<&'static str, &'static [&'static FileType]> = {}",
        map.build()
    )?;
    writeln!(&mut file, ";")?;
    Ok(())
}

/// Generate the file type source constant name within the `file_type` crate
fn file_type_reference(file_type: &FileType) -> String {
    let file_format = file_type.file_format();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

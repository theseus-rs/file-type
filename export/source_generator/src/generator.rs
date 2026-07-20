use crate::{Source, generate_maps};
use anyhow::{Result, bail};
use file_type::FileType;
use file_type::format::FileFormat;
use file_type::sources;
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tracing::{info, warn};

/// Generate Rust source files for the supplied file formats.
///
/// # Errors
/// Returns an error when no file formats are supplied or generated output cannot be read, removed,
/// or written.
pub fn generate(file_formats: &[FileFormat], source_dir: &Path, dry_run: bool) -> Result<()> {
    let Some(first_file_format) = file_formats.first() else {
        bail!("No file formats");
    };
    let source = format!("{:?}", first_file_format.source_type).to_lowercase();
    remove_stale_source_files(file_formats, source_dir, &source, dry_run)?;

    // Generate the module file
    let mut source_code = vec!["use crate::FileType;".to_string(), String::new()];
    for file_format in file_formats {
        let name = format!("{source}_{}", file_format.id);
        source_code.push(format!("pub(crate) mod {name};"));
    }
    source_code.push(String::new());
    for file_format in file_formats {
        let name = format!("{source}_{}", file_format.id);
        source_code.push(format!("pub(crate) use {name}::{};", name.to_uppercase()));
    }
    source_code.push(String::new());
    source_code.push("#[doc(hidden)]".to_string());
    source_code.push("pub const FILE_TYPES: &[&FileType] = &[".to_string());
    for file_format in file_formats {
        let name = format!("{source}_{}", file_format.id);
        source_code.push(format!("    &{},", name.to_uppercase()));
    }
    source_code.push("];".to_string());
    source_code.push(String::new());
    let file_name = source_dir.with_extension("rs");
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
        structs.sort_unstable();
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
    let Some(map_source_dir) = source_dir.parent().and_then(Path::parent) else {
        bail!("Invalid source directory: {}", source_dir.display());
    };
    let source_type = &first_file_format.source_type;
    let mut all_file_formats = sources::file_types()
        .map(FileType::file_format)
        .filter(|file_format| &file_format.source_type != source_type)
        .collect::<Vec<&FileFormat>>();
    all_file_formats.extend(file_formats.iter());
    all_file_formats.sort();
    generate_maps(&all_file_formats, map_source_dir, dry_run)
}

fn remove_stale_source_files(
    file_formats: &[FileFormat],
    source_dir: &Path,
    source: &str,
    dry_run: bool,
) -> Result<()> {
    let expected_files = file_formats
        .iter()
        .map(|file_format| format!("{source}_{}.rs", file_format.id))
        .collect::<BTreeSet<String>>();
    let prefix = format!("{source}_");

    for entry in fs::read_dir(source_dir)? {
        let path = entry?.path();
        let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) else {
            continue;
        };
        if !path.is_file()
            || !file_name.starts_with(&prefix)
            || !path
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("rs"))
            || expected_files.contains(file_name)
        {
            continue;
        }

        if dry_run {
            warn!("[dry-run] Removing {}", path.to_string_lossy());
        } else {
            info!("Removing {}", path.to_string_lossy());
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_maps;
    use file_type::format::SourceType;
    use std::env;

    #[test]
    fn test_generate_errors() {
        assert!(generate(&[], Path::new("."), true).is_err());
        assert!(generate_maps(&[], Path::new("."), true).is_err());

        let file_format = FileFormat::default();
        assert!(generate(&[file_format], Path::new("/tmp"), true).is_err());
    }

    #[test]
    fn test_generate_writes_sources_and_removes_stale_files() {
        let root = env::temp_dir().join(format!(
            "file-type-source-generator-test-{}",
            std::process::id()
        ));
        let source_dir = root.join("src").join("sources").join("default");
        assert!(fs::create_dir_all(&source_dir).is_ok());

        let stale_file = source_dir.join("default_999.rs");
        assert!(fs::write(&stale_file, "stale").is_ok());
        let file_format = FileFormat {
            id: 42,
            source_type: SourceType::Default,
            name: "Test format",
            ..Default::default()
        };

        assert!(generate(&[file_format], &source_dir, false).is_ok());
        assert!(!stale_file.exists());
        assert!(source_dir.with_extension("rs").exists());
        assert!(source_dir.join("default_42.rs").exists());
        assert!(root.join("src").join("signatures.rs").exists());
        assert!(root.join("src").join("extensions.rs").exists());
        assert!(root.join("src").join("media_types.rs").exists());
        assert!(fs::remove_dir_all(root).is_ok());
    }
}

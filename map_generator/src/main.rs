//! # Generate Maps
//!
//! This crate generates the maps used by the `file_type` module to reduce build times and increase
//! runtime performance.

use anyhow::Result;
use file_type::FileType;
use file_type::format::FileFormat;
use file_type::sources::file_types;
use source_generator::generate_maps;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let crate_dir = env::var("CARGO_MANIFEST_DIR")?;
    let source_dir = PathBuf::from(crate_dir)
        .join("..")
        .join("file_type")
        .join("src");
    let file_formats = file_types()
        .map(FileType::file_format)
        .collect::<Vec<&FileFormat>>();

    generate_maps(&file_formats, &source_dir, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

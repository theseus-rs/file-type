use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126951815: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_815,
        source_type: SourceType::Wikidata,
        name: "Rust source code file",
        extensions: &["rs"],
        media_types: &["text/rust", "text/x-rust"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59962003: FileType = FileType {
    file_format: &FileFormat {
        id: 59_962_003,
        source_type: SourceType::Wikidata,
        name: "ScanIt Document",
        extensions: &["sid"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

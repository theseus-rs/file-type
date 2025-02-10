use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113557082: FileType = FileType {
    file_format: &FileFormat {
        id: 113_557_082,
        source_type: SourceType::Wikidata,
        name: "Creator Image format",
        extensions: &["cif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

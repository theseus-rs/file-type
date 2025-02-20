use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650547: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_547,
        source_type: SourceType::Wikidata,
        name: "Package",
        extensions: &["pack"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

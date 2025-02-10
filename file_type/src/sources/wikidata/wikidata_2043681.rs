use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2043681: FileType = FileType {
    file_format: &FileFormat {
        id: 2_043_681,
        source_type: SourceType::Wikidata,
        name: "PAK",
        extensions: &["pak"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

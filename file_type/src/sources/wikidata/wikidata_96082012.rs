use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_96082012: FileType = FileType {
    file_format: &FileFormat {
        id: 96_082_012,
        source_type: SourceType::Wikidata,
        name: "Standard Product Version 3 format",
        extensions: &["sp3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

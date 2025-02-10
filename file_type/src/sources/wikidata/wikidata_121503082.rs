use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121503082: FileType = FileType {
    file_format: &FileFormat {
        id: 121_503_082,
        source_type: SourceType::Wikidata,
        name: "Image Systems CCITT Group 4 file",
        extensions: &["ig4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

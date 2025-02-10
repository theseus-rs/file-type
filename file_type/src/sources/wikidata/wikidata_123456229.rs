use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123456229: FileType = FileType {
    file_format: &FileFormat {
        id: 123_456_229,
        source_type: SourceType::Wikidata,
        name: "CFW Form file",
        extensions: &["cfw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

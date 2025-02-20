use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123385570: FileType = FileType {
    file_format: &FileFormat {
        id: 123_385_570,
        source_type: SourceType::Wikidata,
        name: "Scene library file",
        extensions: &["scl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

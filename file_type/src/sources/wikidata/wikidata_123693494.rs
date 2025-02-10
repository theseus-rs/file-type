use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123693494: FileType = FileType {
    file_format: &FileFormat {
        id: 123_693_494,
        source_type: SourceType::Wikidata,
        name: "Module Definition file",
        extensions: &["def"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123693352: FileType = FileType {
    file_format: &FileFormat {
        id: 123_693_352,
        source_type: SourceType::Wikidata,
        name: "C++ Builder Unit",
        extensions: &["ccp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

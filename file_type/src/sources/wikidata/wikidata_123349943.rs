use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123349943: FileType = FileType {
    file_format: &FileFormat {
        id: 123_349_943,
        source_type: SourceType::Wikidata,
        name: "Family Origins Database",
        extensions: &["fow"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

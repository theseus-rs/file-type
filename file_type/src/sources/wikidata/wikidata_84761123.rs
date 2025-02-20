use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_84761123: FileType = FileType {
    file_format: &FileFormat {
        id: 84_761_123,
        source_type: SourceType::Wikidata,
        name: ".gn",
        extensions: &["gn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

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

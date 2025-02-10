use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000578: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_578,
        source_type: SourceType::Wikidata,
        name: "Android Resource file",
        extensions: &["arsc", "xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

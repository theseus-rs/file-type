use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000485: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_485,
        source_type: SourceType::Wikidata,
        name: "010 Editor Binary Template",
        extensions: &["bt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

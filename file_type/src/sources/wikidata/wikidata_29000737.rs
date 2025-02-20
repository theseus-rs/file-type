use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000737: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_737,
        source_type: SourceType::Wikidata,
        name: "Volume data format",
        extensions: &["vol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

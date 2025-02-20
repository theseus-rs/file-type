use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34748290: FileType = FileType {
    file_format: &FileFormat {
        id: 34_748_290,
        source_type: SourceType::Wikidata,
        name: "T64",
        extensions: &["t64"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

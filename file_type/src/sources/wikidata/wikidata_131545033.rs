use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131545033: FileType = FileType {
    file_format: &FileFormat {
        id: 131_545_033,
        source_type: SourceType::Wikidata,
        name: "Stanford Exploration Project file",
        extensions: &["h"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113621513: FileType = FileType {
    file_format: &FileFormat {
        id: 113_621_513,
        source_type: SourceType::Wikidata,
        name: "Load Runner Scenario file",
        extensions: &["lrs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

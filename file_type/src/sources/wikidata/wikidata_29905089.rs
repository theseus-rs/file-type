use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29905089: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_089,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System consolidation database file",
        extensions: &["sas7bfdb", "sf2", "sf7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

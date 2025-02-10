use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113846496: FileType = FileType {
    file_format: &FileFormat {
        id: 113_846_496,
        source_type: SourceType::Wikidata,
        name: "SureThing Template",
        extensions: &["stt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113435494: FileType = FileType {
    file_format: &FileFormat {
        id: 113_435_494,
        source_type: SourceType::Wikidata,
        name: "Garmin track log file",
        extensions: &["gmn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

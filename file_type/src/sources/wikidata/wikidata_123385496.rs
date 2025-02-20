use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123385496: FileType = FileType {
    file_format: &FileFormat {
        id: 123_385_496,
        source_type: SourceType::Wikidata,
        name: "Path library file",
        extensions: &["ptl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

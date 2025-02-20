use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111355025: FileType = FileType {
    file_format: &FileFormat {
        id: 111_355_025,
        source_type: SourceType::Wikidata,
        name: "exponential 8-bit format",
        extensions: &["u255law"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

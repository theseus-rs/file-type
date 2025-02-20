use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123686339: FileType = FileType {
    file_format: &FileFormat {
        id: 123_686_339,
        source_type: SourceType::Wikidata,
        name: "Digital Negative, version 1.7",
        extensions: &["dng"],
        media_types: &["image/dng"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29904541: FileType = FileType {
    file_format: &FileFormat {
        id: 29_904_541,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System audit file",
        extensions: &["sas7baud", "st7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

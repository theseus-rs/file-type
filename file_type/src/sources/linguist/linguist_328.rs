use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_328: FileType = FileType {
    file_format: &FileFormat {
        id: 328,
        source_type: SourceType::Linguist,
        name: "SAS",
        extensions: &["sas"],
        media_types: &["text/x-sas"],
        signatures: &[],
        related_formats: &[],
    },
};

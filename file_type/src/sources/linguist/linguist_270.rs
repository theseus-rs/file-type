use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_270: FileType = FileType {
    file_format: &FileFormat {
        id: 270,
        source_type: SourceType::Linguist,
        name: "Oz",
        extensions: &["oz"],
        media_types: &["text/x-oz"],
        signatures: &[],
        related_formats: &[],
    },
};

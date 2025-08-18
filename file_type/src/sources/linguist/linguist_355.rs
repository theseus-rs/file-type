use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_355: FileType = FileType {
    file_format: &FileFormat {
        id: 355,
        source_type: SourceType::Linguist,
        name: "Squirrel",
        extensions: &["nut"],
        media_types: &["text/x-squirrel"],
        signatures: &[],
        related_formats: &[],
    },
};

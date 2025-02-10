use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_355: FileType = FileType {
    file_format: &FileFormat {
        id: 355,
        source_type: SourceType::Linguist,
        name: "Squirrel",
        extensions: &["nut"],
        media_types: &["text/x-c++src"],
        signatures: &[],
        related_formats: &[],
    },
};

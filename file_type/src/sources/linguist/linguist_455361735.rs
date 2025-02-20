use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_455361735: FileType = FileType {
    file_format: &FileFormat {
        id: 455_361_735,
        source_type: SourceType::Linguist,
        name: "STL",
        extensions: &["stl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_84359046: FileType = FileType {
    file_format: &FileFormat {
        id: 84_359_046,
        source_type: SourceType::Linguist,
        name: "BuildStream",
        extensions: &["bst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

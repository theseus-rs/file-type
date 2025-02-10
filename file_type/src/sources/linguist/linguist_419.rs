use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_419: FileType = FileType {
    file_format: &FileFormat {
        id: 419,
        source_type: SourceType::Linguist,
        name: "reStructuredText",
        extensions: &["rest", "rest.txt", "rst", "rst.txt"],
        media_types: &["text/x-rst"],
        signatures: &[],
        related_formats: &[],
    },
};

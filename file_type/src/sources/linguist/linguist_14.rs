use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_14: FileType = FileType {
    file_format: &FileFormat {
        id: 14,
        source_type: SourceType::Linguist,
        name: "Alpine Abuild",
        extensions: &[],
        media_types: &["text/x-sh"],
        signatures: &[],
        related_formats: &[],
    },
};

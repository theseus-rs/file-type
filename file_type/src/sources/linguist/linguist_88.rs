use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_88: FileType = FileType {
    file_format: &FileFormat {
        id: 88,
        source_type: SourceType::Linguist,
        name: "Diff",
        extensions: &["diff", "patch"],
        media_types: &["text/x-diff"],
        signatures: &[],
        related_formats: &[],
    },
};

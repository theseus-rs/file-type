use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_85: FileType = FileType {
    file_format: &FileFormat {
        id: 85,
        source_type: SourceType::Linguist,
        name: "DTrace",
        extensions: &["d"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_769248603: FileType = FileType {
    file_format: &FileFormat {
        id: 769_248_603,
        source_type: SourceType::Linguist,
        name: "C3",
        extensions: &["c3"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};

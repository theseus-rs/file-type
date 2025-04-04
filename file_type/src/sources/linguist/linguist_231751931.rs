use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_231751931: FileType = FileType {
    file_format: &FileFormat {
        id: 231_751_931,
        source_type: SourceType::Linguist,
        name: "Monkey C",
        extensions: &["mc"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};

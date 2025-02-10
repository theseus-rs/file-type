use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_182: FileType = FileType {
    file_format: &FileFormat {
        id: 182,
        source_type: SourceType::Linguist,
        name: "Java Server Pages",
        extensions: &["jsp", "tag"],
        media_types: &["application/x-jsp"],
        signatures: &[],
        related_formats: &[],
    },
};

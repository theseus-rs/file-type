use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_403: FileType = FileType {
    file_format: &FileFormat {
        id: 403,
        source_type: SourceType::Linguist,
        name: "XS",
        extensions: &["xs"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};

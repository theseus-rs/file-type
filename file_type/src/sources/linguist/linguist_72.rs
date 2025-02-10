use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_72: FileType = FileType {
    file_format: &FileFormat {
        id: 72,
        source_type: SourceType::Linguist,
        name: "Crystal",
        extensions: &["cr"],
        media_types: &["text/x-crystal"],
        signatures: &[],
        related_formats: &[],
    },
};

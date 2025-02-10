use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_54: FileType = FileType {
    file_format: &FileFormat {
        id: 54,
        source_type: SourceType::Linguist,
        name: "Ceylon",
        extensions: &["ceylon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

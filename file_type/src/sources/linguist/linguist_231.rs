use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_231: FileType = FileType {
    file_format: &FileFormat {
        id: 231,
        source_type: SourceType::Linguist,
        name: "MiniD",
        extensions: &["minid"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
